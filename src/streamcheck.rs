#![allow(non_snake_case)]
use request::Request;

use std::fmt;
use playlist_decoder;
use url::Url;
use hls_m3u8::MasterPlaylist;
//use hls_m3u8::types::QuotedString;

#[derive(Debug)]
pub struct StreamInfo {
    pub Name: String,
    pub Description: String,
    pub Type: String,
    pub Url: String,
    pub Homepage: String,
    pub Genre: String,
    pub Bitrate: u32,
    pub Sampling: u32,
    pub Codec: String,
    pub Hls: bool,
}

#[derive(Debug)]
pub struct StreamCheckError {
    pub Url: String,
    pub Msg: String,
}

impl StreamCheckError {
    fn new(url: &str, msg: &str) -> StreamCheckError {
        StreamCheckError {
            Url: url.to_string(),
            Msg: msg.to_string(),
        }
    }
}

impl fmt::Display for StreamCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.Msg)
    }
}

impl Error for StreamCheckError {
    fn description(&self) -> &str {
        &self.Msg
    }
}

use std::error::Error;
pub type StreamCheckResult = Result<StreamInfo, StreamCheckError>;

fn type_is_m3u(content_type: &str) -> bool {
    return content_type == "application/mpegurl" || content_type == "application/x-mpegurl"
        || content_type == "audio/mpegurl" || content_type == "audio/x-mpegurl"
        || content_type == "application/vnd.apple.mpegurl"
        || content_type == "application/vnd.apple.mpegurl.audio";
}

fn type_is_pls(content_type: &str) -> bool {
    return content_type == "audio/x-scpls" || content_type == "application/x-scpls"
        || content_type == "application/pls+xml";
}

fn type_is_asx(content_type: &str) -> bool {
    return content_type == "video/x-ms-asx" || content_type == "video/x-ms-asf";
}

fn type_is_xspf(content_type: &str) -> bool {
    return content_type == "application/xspf+xml";
}

fn type_is_playlist(content_type: &str) -> bool {
    let search = content_type.find(";");
    let mut content_type = content_type;
    match search {
        Some(index) => {
            content_type = &content_type[0..index];
        }
        None => {}
    }
    return type_is_m3u(content_type) || type_is_pls(content_type) || type_is_asx(content_type)
        || type_is_xspf(content_type);
}

fn type_is_stream(content_type: &str) -> Option<&str> {
    match content_type {
        "audio/mpeg" => Some("MP3"),
        "audio/x-mpeg" => Some("MP3"),
        "audio/mp3" => Some("MP3"),
        "audio/aac" => Some("AAC"),
        "audio/x-aac" => Some("AAC"),
        "audio/aacp" => Some("AAC+"),
        "audio/ogg" => Some("OGG"),
        "application/ogg" => Some("OGG"),
        "audio/flac" => Some("FLAC"),
        "application/flv" => Some("FLV"),
        "application/octet-stream" => Some("UNKNOWN"),
        _ => None,
    }
}

pub fn check(url: &str, check_all: bool, timeout: u32, max_depth: u8) -> Vec<StreamCheckResult> {
    if max_depth == 0{
        return vec![Err(StreamCheckError::new(url, "max depth reached"))];
    }
    let request = Request::new(&url, "StreamCheckBot/0.1.0", timeout);
    let mut list: Vec<StreamCheckResult> = vec![];
    match request {
        Ok(mut request) => {
            if request.info.code >= 200 && request.info.code < 300 {
                let mut is_playlist = false;
                let mut is_stream = false;
                let mut stream_type = String::from("");
                {
                    let content_type = request.info.headers.get("content-type");
                    match content_type {
                        Some(content_type) => {
                            let content_type_lower = content_type.to_lowercase();
                            if type_is_playlist(&content_type_lower) {
                                is_playlist = true;
                            } else if type_is_stream(&content_type_lower).is_some() {
                                stream_type =
                                    String::from(type_is_stream(&content_type_lower).unwrap_or(""));
                                is_stream = true;
                            } else {
                                list.push(Err(StreamCheckError::new(
                                    url,
                                    &format!("unknown content type {}", content_type_lower),
                                )));
                            }
                        }
                        None => {
                            list.push(Err(StreamCheckError::new(
                                url,
                                "Missing content-type in http header",
                            )));
                        }
                    }
                }
                if is_playlist {
                    let read_result = request.read_content();
                    match read_result {
                        Ok(_)=>{
                            let content = request.get_content();
                            let is_hls = playlist_decoder::is_content_hls(&content);
                            if is_hls {
                                let playlist = content.parse::<MasterPlaylist>();
                                match playlist{
                                    Ok(playlist)=>{
                                        for i in playlist.stream_inf_tags() {
                                            let mut codecs: String = String::new();
                                            /*let codecs_obj = i.codecs();
                                            if let Some(codecs_obj) = codecs_obj {
                                                codecs = codecs_obj.to_string();
                                            }*/
                                            let stream = StreamInfo {
                                                Url: String::from(url),
                                                Type: String::from(""),
                                                Name: String::from(""),
                                                Description: String::from(""),
                                                Homepage: String::from(""),
                                                Bitrate: i.bandwidth() as u32,
                                                Genre: String::from(""),
                                                Sampling: 0,
                                                Codec: codecs,
                                                Hls: true,
                                            };
                                            //println!("{:?}", stream);
                                            list.push(Ok(stream));
                                            break;
                                        }
                                    }
                                    Err(_)=>{
                                        let stream = StreamInfo {
                                            Url: String::from(url),
                                            Type: String::from(""),
                                            Name: String::from(""),
                                            Description: String::from(""),
                                            Homepage: String::from(""),
                                            Bitrate: 0,
                                            Genre: String::from(""),
                                            Sampling: 0,
                                            Codec: String::from("UNKNOWN"),
                                            Hls: true,
                                        };
                                        list.push(Ok(stream));
                                    }
                                }
                            }else{
                                let playlist = decode_playlist(url, &content,check_all, timeout, max_depth - 1);
                                if playlist.len() == 0 {
                                    list.push(Err(StreamCheckError::new(url, "Empty playlist")));
                                } else {
                                    list.extend(playlist);
                                }
                            }
                        }
                        Err(err)=>{
                            list.push(Err(StreamCheckError::new(url, &err.to_string())));
                        }
                    }
                } else if is_stream {
                    let headers = request.info.headers;
                    let stream = StreamInfo {
                        Url: String::from(url),
                        Type: headers
                            .get("content-type")
                            .unwrap_or(&String::from(""))
                            .clone(),
                        Name: headers.get("icy-name").unwrap_or(&String::from("")).clone(),
                        Description: headers
                            .get("icy-description")
                            .unwrap_or(&String::from(""))
                            .clone(),
                        Homepage: headers.get("icy-url").unwrap_or(&String::from("")).clone(),
                        Bitrate: headers
                            .get("icy-br")
                            .unwrap_or(&String::from(""))
                            .parse()
                            .unwrap_or(0),
                        Genre: headers
                            .get("icy-genre")
                            .unwrap_or(&String::from(""))
                            .clone(),
                        Sampling: headers
                            .get("icy-sr")
                            .unwrap_or(&String::from(""))
                            .parse()
                            .unwrap_or(0),
                        Codec: stream_type,
                        Hls: false,
                    };
                    list.push(Ok(stream));
                }
            } else if request.info.code >= 300 && request.info.code < 400 {
                let location = request.info.headers.get("location");
                match location {
                    Some(location) => {
                        list.extend(check(location,check_all, timeout,max_depth - 1));
                    }
                    None => {}
                }
            } else {
                list.push(Err(StreamCheckError::new(
                    url,
                    &format!("illegal http status code {}", request.info.code),
                )));
            }
        }
        Err(err) => list.push(Err(StreamCheckError::new(url, &err.to_string()))),
    }
    list
}

fn decode_playlist(url_str: &str, content: &str, check_all: bool, timeout: u32, max_depth: u8) -> Vec<StreamCheckResult> {
    let mut list = vec![];
    let base_url = Url::parse(url_str);
    match base_url {
        Ok(base_url) => {
            let urls = playlist_decoder::decode(content);
            let mut max_urls = 10;
            for url in urls {
                max_urls = max_urls - 1;
                if max_urls == 0{
                    break;
                }
                if url.trim() != "" {
                    let abs_url = base_url.join(&url);
                    match abs_url {
                        Ok(abs_url) => {
                            let result = check(&abs_url.as_str(),check_all, timeout, max_depth);
                            if !check_all{
                                let mut found = false;
                                for result_single in result.iter() {
                                    if result_single.is_ok() {
                                        found = true;
                                    }
                                }
                                if found{
                                    list.extend(result);
                                    break;
                                }
                            }
                            list.extend(result);
                        }
                        Err(err) => {
                            list.push(Err(StreamCheckError::new(
                                url_str,
                                &err.to_string(),
                            )));
                        }
                    }
                }
            }
        }
        Err(err) => {
            list.push(Err(StreamCheckError::new(
                url_str,
                &err.to_string(),
            )));
        }
    }

    list
}
