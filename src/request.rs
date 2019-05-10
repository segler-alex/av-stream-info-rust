use native_tls::TlsConnector;

use std::fmt;

use std::net::TcpStream;
use std::net::SocketAddr;
use std::io::{Read, Write};

use std::error::Error;
use url::Url;
use std::collections::HashMap;

type BoxResult<T> = Result<T, Box<Error>>;

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
    readable: Box<Read>,
    content_read_done: bool,
    content: String,
}

use std::time::Duration;
use std::net::ToSocketAddrs;
use std::vec::IntoIter;

fn connect(addrs: Box<IntoIter<SocketAddr>>, timeout: u32) -> BoxResult<TcpStream> {
    for addr in addrs {
        let mut stream = TcpStream::connect_timeout(
            &addr,
            Duration::from_secs(timeout as u64),
        );
        if let Ok(stream) = stream {
            return Ok(stream);
        }
    }
    return Err(Box::new(RequestError::new("connection was not possible")));
}

impl Request {
    pub fn new(url_str: &str, agent: &str, timeout: u32) -> BoxResult<Request> {
        let url = Url::parse(url_str)?;

        let host = url.host_str()
            .ok_or(RequestError::new("illegal host name"))?;
        let port = url.port_or_known_default()
            .ok_or(RequestError::new("port unknown"))?;

        let connect_str = format!("{}:{}", host, port);
        let addrs_iter = connect_str.to_socket_addrs()?;
        let mut stream: TcpStream = connect(Box::new(addrs_iter),timeout)?;
        stream.set_read_timeout(Some(Duration::from_secs(timeout as u64)))?;

        if url.scheme() == "https" {
            let connector = TlsConnector::builder().build()?;
            let mut stream = connector.connect(host, stream)?;
            let mut host_str = String::from(host);
            if port != 443{
                host_str = format!("{}:{}",host,port);
            }
            let query = url.query();
            if let Some(query) = query {
                let full_path = format!("{}?{}",url.path(),query);
                Request::send_request(agent, &mut stream, &host_str, &full_path)?;
            }else{
                Request::send_request(agent, &mut stream, &host_str, url.path())?;
            }
            let header = Request::read_request(&mut stream)?;
            Ok(Request {
                info: header,
                readable: Box::new(stream),
                content_read_done: false,
                content: String::from(""),
            })
        } else if url.scheme() == "http" {
            let mut host_str = String::from(host);
            if port != 80{
                host_str = format!("{}:{}",host,port);
            }
            let query = url.query();
            if let Some(query) = query {
                let full_path = format!("{}?{}",url.path(),query);
                Request::send_request(agent, &mut stream, &host_str, &full_path)?;
            }else{
                Request::send_request(agent, &mut stream, &host_str, url.path())?;
            }
            let header = Request::read_request(&mut stream)?;
            Ok(Request {
                info: header,
                readable: Box::new(stream),
                content_read_done: false,
                content: String::from(""),
            })
        } else {
            Err(Box::new(RequestError::new("unknown scheme")))
        }
    }

    pub fn read_content(&mut self) -> BoxResult<()> {
        if self.content_read_done {
            return Ok(());
        }
        self.content_read_done = true;

        let content_length = self.info
            .headers
            .get("content-length")
            .unwrap_or(&String::from(""))
            .parse();
        match content_length {
            Ok(content_length) => {
                let mut buffer = vec![0; content_length];
                self.readable.read_exact(&mut buffer)?;
                //let out = String::from_utf8(buffer)?;
                let out = String::from_utf8_lossy(&buffer);
                self.content = out.to_string();
                return Ok(());
            }
            Err(_) => {
                let mut result_buffer = vec![];
                loop {
                    let mut buffer = vec![0; 10000];
                    let result = self.readable.read(&mut buffer);

                    match result {
                        Ok(bytes) => {
                            if bytes == 0 {
                                break;
                            } else {
                                result_buffer.extend(buffer[0..bytes].iter());
                            }
                        }
                        Err(err) => {
                            println!("err {}", err);
                        }
                    }

                    if result_buffer.len() > 10000 {
                        break;
                    }
                }
                //let out = String::from_utf8(result_buffer)?;
                let out = String::from_utf8_lossy(&result_buffer);
                self.content = out.to_string();
                return Ok(());
            }
        }
    }

    pub fn get_content<'a>(&'a self) -> &'a str {
        &self.content
    }

    fn read_stream_until(stream: &mut Read, condition: &'static [u8]) -> BoxResult<String> {
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

    fn send_request(agent: &str, stream: &mut Write, host: &str, path: &str) -> BoxResult<()> {
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

    fn read_request(stream: &mut Read) -> BoxResult<HttpHeaders> {
        let out = Request::read_stream_until(stream, b"\r\n")?;
        let mut httpinfo = Request::decode_first_line(&out)?;

        let out = Request::read_stream_until(stream, b"\r\n\r\n")?;
        let lines = out.lines();

        for line in lines {
            match line.find(':') {
                Some(index) => {
                    let (key, value) = line.split_at(index);
                    httpinfo.headers.insert(
                        String::from(key).to_lowercase(),
                        String::from(value[1..].trim()),
                    );
                }
                _ => {}
            }
        }
        Ok(httpinfo)
    }
}
