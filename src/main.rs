#![allow(dead_code)]

use std::collections::HashMap;
use std::net::TcpListener;
use std::thread;

mod args;
mod handler;

use args::parse_args;
use handler::handle_stream;

fn main() {
    let args = parse_args();

    let addr = format!("{}:{}", args.ipaddr, args.port);
    println!("running on port: {}", args.port);
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        if let Ok(mut s) = stream {
            println!("Accepted new connection");
            thread::spawn(move || {
                let mut exp_map: HashMap<String, u64> = HashMap::new();
                let mut val_map: HashMap<String, String> = HashMap::new();
                while s.peer_addr().is_ok() {
                    let res = handle_stream(&mut s, &mut val_map, &mut exp_map);
                    if res == None {
                        break;
                    };
                }
            });
        }
    }
}
