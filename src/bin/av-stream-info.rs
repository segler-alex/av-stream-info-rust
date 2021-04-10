use std::env;
use av_stream_info_rust::check_tree;
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init();

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

    /*
    println!("TCP_TIMEOUT   : {}", tcp_timeout);
    println!("MAX_DEPTH     : {}", max_depth);
    println!("RETRIES       : {}", retries);
    */

    match env::args().nth(1) {
        Some(url) => {
            let item = check_tree(&url, tcp_timeout, max_depth, retries, true);
            println!("{:#?}", item);
        }
        None => {
            eprintln!("1 parameter needed");
        }
    };
}