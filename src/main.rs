use std::net::UdpSocket;
use plotly::{ Plot, Scatter };

use std::io::{ Write };

use std::thread;

mod algess;
use crate::algess::cmd::{ start_recv_stdin };
use crate::algess::server::{ listen };

fn main() {
    let mut table: Vec<(f64, f64)> = vec![];
    let sender = UdpSocket::bind("127.0.0.1:0").expect("error: ");
    
    listen().join().expect("UDP Listen server thread panicked.");
    println!("\nSuccess to starting server.\n");
    start_recv_stdin();
}

fn draw_graph(table: &Vec<(f64, f64)>) {
    let mut plot = Plot::new();

    let mut sorted = table.clone();
    sorted.sort_by(|a, b| a.0.total_cmp(&(b.0)));

    let times: Vec<f64> = sorted.iter().map(|(time, _)| *time).collect();
    let values: Vec<f64> = sorted.iter().map(|(_, value)| *value).collect();

    let trace = Scatter::new(times, values);
    plot.add_trace(trace);

    plot.write_html("out.html");
    println!("Done.");
}