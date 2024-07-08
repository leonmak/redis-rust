#![allow(dead_code)]

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn is_connection_open(stream: &TcpStream) -> bool {
    stream.peer_addr().is_ok()
}

fn next_part(iter: &mut std::str::Split<&str>) -> String {
    let _ = iter.next(); // ignore cmd_len, e.g. $4 for PING
    iter.next().unwrap().to_owned()
}

fn handle_stream(stream: &mut TcpStream, map_store: &mut HashMap<String, String>) -> Option<usize> {
    // Note: redis-cli does not send EOF, so it is blocked on stream.read()

    // stream.read_to_string(&mut buffer)?;

    // Buffered Read - copy to vec then convert to string
    let mut data = Vec::new();
    let mut buf = [0u8; 50];
    let mut bytes_read = 0;
    println!("Reading data...");
    loop {
        let n = stream.read(&mut buf).ok()?;
        println!("{} bytes read", n);
        if n == 0 {
            return None;
        }
        bytes_read += n;
        data.extend_from_slice(&buf[..n]);

        // stop reading if the \r\n is sent
        if &data[data.len() - 2..] == &[b'\r', b'\n'] {
            break;
        }
        // println!("last two: {:?}", last_two);
    }
    println!("{:?} bytes read", bytes_read);
    // println!("buffer: {:?}", String::from_utf8(Vec::from(buf)));

    // Split and process each line
    let buffer = String::from_utf8(data).unwrap();
    let mut bytes_written = 0;
    let mut iter = buffer.split("\r\n");
    // e.g. *12 = 12 commands
    let mut cmd_lines = iter.next().unwrap()[1..].parse::<i32>().unwrap();
    println!("NUM_LINES, {}", cmd_lines);

    // Parse command
    let cmd = next_part(&mut iter);
    let cmd_str = cmd.as_str();
    let mut resp_s = String::new();
    println!("CMD: {}", cmd_str);

    if cmd_str == "PING" {
        resp_s = "+PONG\r\n".to_owned();
    }

    // Parse args
    cmd_lines -= 1;
    while cmd_lines > 0 {
        cmd_lines -= 1;
        let arg = next_part(&mut iter);
        println!("ARG: {}", arg);
        let resp_s_arg = match cmd_str {
            "ECHO" => arg,
            "GET" => {
                let val = map_store.get(&arg).expect("NO VALUE");
                val.to_owned()
            }
            "SET" => {
                let val = next_part(&mut iter);
                println!("VAL: {}", val);
                cmd_lines -= 1;
                map_store.insert(arg, val);
                "OK".to_owned()
            }
            _ => "OK".to_owned(),
        };
        let res_str = format!("+{}\r\n", resp_s_arg);
        resp_s.push_str(&res_str);
    }
    println!(">> {}", resp_s);
    bytes_written += stream.write(resp_s.as_bytes()).unwrap();

    return Some(bytes_written);
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut s) = stream {
            println!("Accepted new connection");
            thread::spawn(move || {
                let mut map: HashMap<String, String> = HashMap::new();
                while is_connection_open(&s) {
                    if handle_stream(&mut s, &mut map) == None {
                        break;
                    };
                }
            });
        }
    }
}
