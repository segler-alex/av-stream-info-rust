use native_tls::TlsConnector;

use std::fmt;

use std::io::{Read, Write};
use std::net::SocketAddr;
use std::net::TcpStream;

use std::collections::HashMap;
use std::error::Error;
use url::Url;

type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct RequestError {
    details: String,
}

impl RequestError {
    fn new(msg: &str) -> RequestError {
        RequestError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for RequestError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct HttpHeaders {
    pub code: u32,
    pub message: String,
    pub version: String,
    pub headers: HashMap<String, String>,
}

pub struct Request {
    pub info: HttpHeaders,
    readable: Box<dyn Read>,
    content_read_done: bool,
    content_vec: Vec<u8>,
    ssl_error: bool,
}

use std::net::ToSocketAddrs;
use std::time::Duration;
use std::vec::IntoIter;

fn connect(addrs: Box<IntoIter<SocketAddr>>, timeout: u32) -> BoxResult<TcpStream> {
    for addr in addrs {
        let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(timeout as u64));
        if let Ok(stream) = stream {
            return Ok(stream);
        }
    }
    return Err(Box::new(RequestError::new("connection was not possible")));
}

impl Request {
    pub fn new(url_str: &str, agent: &str, timeout: u32) -> BoxResult<Request> {
        let url = Url::parse(url_str)?;

        let host = url
            .host_str()
            .ok_or(RequestError::new("illegal host name"))?;
        let port = url
            .port_or_known_default()
            .ok_or(RequestError::new("port unknown"))?;

        let connect_str = format!("{}:{}", host, port);
        let addrs_iter = connect_str.to_socket_addrs()?;
        let mut stream: TcpStream = connect(Box::new(addrs_iter), timeout)?;
        stream.set_read_timeout(Some(Duration::from_secs(timeout as u64)))?;

        if url.scheme() == "https" {
            let mut connector = TlsConnector::builder().build()?;
            let mut ssl_error = false;
            let mut sslstream = connector.connect(host, stream);
            if sslstream.is_err() {
                // retry connection on error with settings
                // to ignore ssl errors
                // return that we have done so
                let addrs_iter = connect_str.to_socket_addrs()?;
                let stream: TcpStream = connect(Box::new(addrs_iter), timeout)?;
                stream.set_read_timeout(Some(Duration::from_secs(timeout as u64)))?;
                connector = TlsConnector::builder()
                    .danger_accept_invalid_certs(true)
                    .danger_accept_invalid_hostnames(true)
                    .build()?;
                    sslstream = connector.connect(host, stream);
                ssl_error = true;
            }
            let mut sslstream = sslstream?;
            let mut host_str = String::from(host);
            if port != 443 {
                host_str = format!("{}:{}", host, port);
            }
            let query = url.query();
            if let Some(query) = query {
                let full_path = format!("{}?{}", url.path(), query);
                Request::send_request(agent, &mut sslstream, &host_str, &full_path)?;
            } else {
                Request::send_request(agent, &mut sslstream, &host_str, url.path())?;
            }
            let header = Request::read_request(&mut sslstream)?;
            Ok(Request {
                info: header,
                readable: Box::new(sslstream),
                content_read_done: false,
                content_vec: vec![],
                ssl_error,
            })
        } else if url.scheme() == "http" {
            let mut host_str = String::from(host);
            if port != 80 {
                host_str = format!("{}:{}", host, port);
            }
            let query = url.query();
            if let Some(query) = query {
                let full_path = format!("{}?{}", url.path(), query);
                Request::send_request(agent, &mut stream, &host_str, &full_path)?;
            } else {
                Request::send_request(agent, &mut stream, &host_str, url.path())?;
            }
            let header = Request::read_request(&mut stream)?;
            Ok(Request {
                info: header,
                readable: Box::new(stream),
                content_read_done: false,
                content_vec: vec![],
                ssl_error: false,
            })
        } else {
            Err(Box::new(RequestError::new("unknown scheme")))
        }
    }

    pub fn read_up_to(&mut self, max_size: usize) -> BoxResult<()> {
        let chunck_size = 10000;
        let mut buffer = vec![0; chunck_size];
        loop {
            if self.content_vec.len() >= max_size {
                break;
            }

            let bytes = self.readable.read(&mut buffer)?;

            if bytes == 0 {
                break;
            } else {
                self.content_vec.extend(buffer[0..bytes].iter());
                buffer.clear();
            }
        }
        Ok(())
    }

    pub fn read_content(&mut self) -> BoxResult<()> {
        if self.content_read_done {
            return Ok(());
        }
        self.content_read_done = true;

        let content_length = self.content_length().unwrap_or(10000);
        self.read_up_to(content_length)?;
        Ok(())
    }

    pub fn content_length(&self) -> BoxResult<usize> {
        let content_length = self
            .info
            .headers
            .get("content-length")
            .unwrap_or(&String::from(""))
            .parse()?;
        Ok(content_length)
    }

    pub fn text<'a>(&'a self) -> String {
        let out = String::from_utf8_lossy(&self.content_vec);
        let content = out.to_string();
        return content.clone();
    }

    //pub fn bytes<'a>(&'a self) -> &'a [u8] {
    //    self.content_vec.as_slice()
    //}

    fn read_stream_until(stream: &mut dyn Read, condition: &'static [u8]) -> BoxResult<String> {
        let mut buffer = vec![0; 1];
        let mut bytes = Vec::new();
        loop {
            let result_recv = stream.read(&mut buffer);
            match result_recv {
                Ok(a) => {
                    if a == 0 {
                        break;
                    } else {
                        bytes.push(buffer[0]);
                        if bytes.len() >= condition.len() {
                            let (_, right) = bytes.split_at(bytes.len() - condition.len());
                            if right == condition {
                                break;
                            }
                        }
                    }
                }
                _ => {
                    break;
                }
            }
            if bytes.len() > 10000 {
                break;
            }
        }
        let out = String::from_utf8_lossy(&bytes);
        Ok(out.to_string())
    }

    fn send_request(agent: &str, stream: &mut dyn Write, host: &str, path: &str) -> BoxResult<()> {
        let request_str = format!(
            "GET {} HTTP/1.0\r\nHost: {}\r\nAccept: */*\r\nUser-Agent: {}\r\nConnection: close\r\n\r\n",
            path, host, agent
        );
        stream.write(request_str.as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    fn decode_first_line(line: &str) -> BoxResult<HttpHeaders> {
        if line.starts_with("HTTP/") {
            if line.len() < 14 {
                return Err(Box::new(RequestError::new("HTTP status line too short")));
            }
            Ok(HttpHeaders {
                code: line[9..12].parse()?,
                message: String::from(&line[13..]),
                version: String::from(&line[5..8]),
                headers: HashMap::new(),
            })
        } else if line.starts_with("ICY") {
            Ok(HttpHeaders {
                code: line[4..7].parse()?,
                message: String::from(&line[8..]),
                version: String::from(""),
                headers: HashMap::new(),
            })
        } else {
            return Err(Box::new(RequestError::new("HTTP header missing")));
        }
    }

    fn read_request(stream: &mut dyn Read) -> BoxResult<HttpHeaders> {
        let out = Request::read_stream_until(stream, b"\r\n")?;
        let mut httpinfo = Request::decode_first_line(&out)?;

        let out = Request::read_stream_until(stream, b"\r\n\r\n")?;
        let lines = out.lines();

        for line in lines {
            match line.find(':') {
                Some(index) => {
                    let (key, value) = line.split_at(index);
                    let key_trimmed = String::from(key).to_lowercase();
                    let value_trimmed = String::from(value[1..].trim());
                    httpinfo
                        .headers
                        .entry(key_trimmed)
                        .and_modify(|s| {
                            s.push_str(",");
                            s.push_str(&value_trimmed);
                        })
                        .or_insert(value_trimmed);
                }
                _ => {}
            }
        }
        Ok(httpinfo)
    }

    pub fn had_ssl_error(&self) -> bool {
        self.ssl_error
    }
}
