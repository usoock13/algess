use std::net::UdpSocket;
use std::thread::{self, JoinHandle};

use crate::algess::channels::ChannelManager;

pub struct Data {
    ch: String,     /* Channel's name */
    l: String,      /* Label */
    x: f64,         /* 2D Data X */
    y: f64,         /* 2D Data Y */
}

/// Threading on background.
pub fn listen() -> JoinHandle<()> {
    thread::spawn(move || {
        let receiver = UdpSocket::bind("127.0.0.1:2025").expect("error: ");
        let mut buf = [0; 128];
        const QUIT_COMMAND: &str = "quit";

        loop {
            let (amt, src) = receiver.recv_from(&mut buf).expect("error: ");
            let buf = &mut buf[..amt];
        
            let s = match str::from_utf8(buf) {
                Ok(v) => v,
                Err(err) => panic!("{}", err),
            };
        
            match s.to_lowercase().as_str() {
                QUIT_COMMAND => {
                    // eprintln!("QUIT");
                    break;
                },
                _ => {
                    eprintln!("RECEIVED >> \n\t{}", s);
                    match json::parse(s) {
                        Ok(parsed) => {
                            let ch: String = match parsed["ch"].as_str() {
                                Some(s) => s.into(),
                                None => String::from("default"),
                            };
                            let l: String = match parsed["l"].as_str() {
                                Some(s) => s.into(),
                                None => String::from(""),
                            };
                            let x: f64 = match parsed["x"].as_number() {
                                Some(n) => n.into(),
                                None => panic!("error: Parsing from JsonValue to f64 was failed with {}.", parsed["x"]),
                            };
                            let y: f64 = match parsed["y"].as_number() {
                                Some(n) => n.into(),
                                None => panic!("error: Parsing from JsonValue to f64 was failed with {}.", parsed["y"]),
                            };
                            handle_data(Data {ch, l, x, y});
                        },
                        Err(err) => {
                            panic!("error: Failed {}", err);
                        }
                    }
                }
            }
        }
    })
}

fn handle_data(data: Data) {
    todo!("데이터 처리 및 맞는 채널에 데이터 추가하는 구현이 필요합니다.");
    // ChannelManager::get_channel(&self, name)
    // println!("count: {}\nlast: ({}, {})", table.iter().count(), x, y);
}