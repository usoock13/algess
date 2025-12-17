use std::net::UdpSocket;
use std::str::FromStr;
use json::number::Number;
use plotly::{ Plot, Scatter };

use std::io::{ Write };

use std::thread;

fn main() {
    const DRAW_COMMAND: &str = "draw";
    const QUIT_COMMAND: &str = "quit";

    let mut table: Vec<(f64, f64)> = vec![];

    let receiver = UdpSocket::bind("127.0.0.1:2025").expect("error: ");
    let sender = UdpSocket::bind("127.0.0.1:0").expect("error: ");
    let mut buf: [u8; 32] = [0; 32];

    let background_thread = thread::spawn(move || {
        loop {
            let (amt, src) = receiver.recv_from(&mut buf).expect("error: ");
            let buf = &mut buf[..amt];
    
            let s = match str::from_utf8(buf) {
                Ok(v) => v,
                Err(err) => panic!("{}", err),
            };
    
            match s {
                QUIT_COMMAND => {
                    eprintln!("QUIT");
                    break;
                },
                DRAW_COMMAND => {
                    println!("Start to draw a graph.");
                    draw_graph(&table);
                },
                _ => {
                    eprintln!("RECEIVED >> \n\t{}", s);
                    match json::parse(s) {
                        Ok(parsed) => {
                            eprintln!("time : {}\nvalue : {}", parsed["time"], parsed["value"]);
                            let time: f64 = match parsed["time"].as_number() {
                                Some(n) => n.into(),
                                None => panic!("error: Parsing from JsonValue to f64 was failed with {}.", parsed["time"]),
                            };
                            let value: f64 = match parsed["value"].as_number() {
                                Some(n) => n.into(),
                                None => panic!("error: Parsing from JsonValue to f64 was failed with {}.", parsed["value"]),
                            };

                            table.push((time, value));
                            println!("count: {}\nlast: ({}, {})", table.iter().count(), time, value);
                        },
                        Err(err) => {
                            panic!("error: Failed {}", err);
                        }
                    }
                }
            }
        }
    });

    println!("Start Server...");
    
    loop {
        let mut input = String::new();
        println!(">> ");
        std::io::stdout().flush().expect("Flush 실패");

        let res = std::io::stdin().read_line(&mut input);
        match res {
            Ok(_) => {
                let timmed_input = input.trim();

                match timmed_input {
                    QUIT_COMMAND => {
                        sender.send_to(QUIT_COMMAND.as_bytes(), "127.0.0.1:2025").expect("error: ");
                        break;
                    },
                    DRAW_COMMAND => {
                        sender.send_to(DRAW_COMMAND.as_bytes(), "127.0.0.1:2025").expect("error: ");
                    }
                    _ => continue,
                }
            },
            Err(error) => panic!("error: {}", error),
        }
    }
    background_thread.join().expect("Background thread panicked");
}

fn draw_graph(table: &Vec<(f64, f64)>) {
    let mut plot = Plot::new();

    let times: Vec<f64> = table.iter().map(|(time, _)| *time).collect();
    let values: Vec<f64> = table.iter().map(|(_, value)| *value).collect();

    let trace = Scatter::new(times, values);
    plot.add_trace(trace);

    plot.write_html("out.html");
    println!("Done.");
}