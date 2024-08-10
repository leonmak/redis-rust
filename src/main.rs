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
    let has_master = !args.master_addr.is_empty();
    if has_master {
        println!("replica of master - {}", args.master_addr);
    };
    for stream in listener.incoming() {
        if let Ok(mut s) = stream {
            println!("Accepted new connection");
            thread::spawn(move || {
                let mut exp_map: HashMap<String, u64> = HashMap::new();
                let mut val_map: HashMap<String, String> = HashMap::new();
                let role = if has_master { "slave" } else { "master" };
                let master_replica_id = "8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb";
                let info_kv = vec![
                    "role",
                    role,
                    "master_replid",
                    master_replica_id,
                    "master_repl_offset",
                    "0",
                ];
                while s.peer_addr().is_ok() {
                    let res = handle_stream(&mut s, &mut val_map, &mut exp_map, &info_kv);
                    if res == None {
                        break;
                    };
                }
            });
        }
    }
}
