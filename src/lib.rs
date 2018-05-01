extern crate native_tls;
extern crate playlist_decoder;
extern crate url;
extern crate hls_m3u8;

mod request;
mod streamcheck;

use std::time::Duration;
use std::thread;

fn watchdog(info: String, seconds: u32){
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(seconds as u64));
        println!("Still not finished: {}", info);
        std::process::exit(0x0100);
    });
}

pub fn check(
    url: &str,
    timeout: u32,
    max_depth: u8,
    retries: u8,
) -> Vec<streamcheck::StreamCheckResult> {
    let maxtimeout: u32 = (retries as u32) * timeout * 2;
    let mut working = false;
    let mut list = vec![];

    watchdog(String::from(url), maxtimeout);
    for _i in 0..retries {
        list = streamcheck::check(url, false, timeout, max_depth);
        for item in list.iter() {
            match item {
                &Ok(_) => {
                    working = true;
                    break;
                }
                &Err(_) => {}
            }
        }

        if working {
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }
    list
}