extern crate colored;
extern crate av_stream_info_rust;

use colored::*;
use std::env;
use av_stream_info_rust::check;

fn main() {
    let tcp_timeout: u32 = env::var("TCP_TIMEOUT")
        .unwrap_or(String::from("10"))
        .parse()
        .expect("TCP_TIMEOUT is not u32");
    let max_depth: u8 = env::var("MAX_DEPTH")
        .unwrap_or(String::from("5"))
        .parse()
        .expect("MAX_DEPTH is not u8");
    let retries: u8 = env::var("RETRIES")
        .unwrap_or(String::from("5"))
        .parse()
        .expect("RETRIES is not u8");

    println!("TCP_TIMEOUT   : {}", tcp_timeout);
    println!("MAX_DEPTH     : {}", max_depth);
    println!("RETRIES       : {}", retries);

    match env::args().nth(1) {
        Some(url) => {
            let list = check(&url, tcp_timeout, max_depth, retries);
            for item in list.iter() {
                match item {
                    &Ok(ref item) => {
                        println!("+ {} Codec='{}' Bitrate='{}' (MSG: {})", item.Url, item.Codec, item.Bitrate, "OK".green());
                        break;
                    }
                    &Err(ref e) => {
                        println!("- {} (MSG: {})", e.Url, e.Msg.red());
                    }
                }
            }
        }
        None => {
            println!("1 parameter needed");
        }
    };
}