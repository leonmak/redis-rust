#![allow(dead_code)]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn is_connection_open(stream: &TcpStream) -> bool {
    stream.peer_addr().is_ok()
}

fn handle_stream(stream: &mut TcpStream) -> Option<usize> {
    // Note: redis-cli does not send EOF, so it is blocked on stream.read()

    // stream.read_to_string(&mut buffer)?;
    // println!("buffer: {}", buffer);

    // Read to vec then convert to string
    let mut data = Vec::new();
    let mut buf = [0u8; 10];
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

    // Split and process each line
    let buffer = String::from_utf8(data).unwrap();
    let mut bytes_written = 0;
    let mut iter = buffer.split("\r\n");
    // e.g. *12 = 12 commands
    let mut num_cmds = iter.next().unwrap()[1..].parse::<i32>().unwrap();
    println!("NUM_CMDS, {}", num_cmds);
    while num_cmds > 0 {
        num_cmds -= 1;
        let _ = iter.next(); // ignore cmd_len, e.g. $4 for PING
        let cmd = iter.next().unwrap();
        println!("CMD: {}", cmd);
        let resp_s = match cmd {
            "PING" => "+PONG\r\n",
            _ => "+OK\r\n",
        };
        println!(">> {}", resp_s);
        bytes_written += stream.write(resp_s.as_bytes()).unwrap();
    }
    return Some(bytes_written);
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut s) = stream {
            println!("Accepted new connection");
            while is_connection_open(&s) {
                if handle_stream(&mut s) == None {
                    break;
                };
            }
        }
    }
}
