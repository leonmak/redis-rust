#![allow(dead_code)]

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

fn is_connection_open(stream: &TcpStream) -> bool {
    stream.peer_addr().is_ok()
}

fn next_part(iter: &mut std::str::Split<&str>) -> String {
    let _ = iter.next(); // ignore cmd_len, e.g. $4 for PING
    iter.next().unwrap().to_owned()
}

fn time_now() -> u64 {
    let now = SystemTime::now();
    let duration_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get duration since Unix epoch");

    let millis = duration_since_epoch.as_millis() as u64;
    millis
}

fn handle_stream(
    stream: &mut TcpStream,
    map_store: &mut HashMap<String, String>,
    exp_store: &mut HashMap<String, u64>,
) -> Option<usize> {
    // Note: redis-cli does not send EOF, so it is blocked on stream.read()

    // stream.read_to_string(&mut buffer)?;

    // Buffered Read - copy to vec then convert to string
    let mut data = Vec::new();
    let mut buf = [0u8; 422];
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

        // stop reading if the \r\n is sent // stops before end with buf match \r\n
        if &data[data.len() - 2..] == &[b'\r', b'\n'] {
            break;
        }
        // println!("last two: {:?}", last_two);
    }
    println!("{:?} bytes read", bytes_read);
    // println!("buffer: {:?}", String::from_utf8(data.clone()));

    // Split and process each line
    let buffer = String::from_utf8(data).unwrap();
    let mut bytes_written = 0;
    let mut iter = buffer.split("\r\n");
    // e.g. *12 = 12 commands
    let mut arg_lines = iter.next().unwrap()[1..].parse::<i32>().unwrap();
    println!("NUM_LINES, {}", arg_lines);

    // Parse command
    let cmd = next_part(&mut iter);
    let cmd_str = cmd.as_str();
    let mut resp_s = String::new();
    println!("CMD: {}", cmd_str);

    if cmd_str == "PING" {
        resp_s = "+PONG\r\n".to_owned();
    }

    // Parse args
    arg_lines -= 1;
    while arg_lines > 0 {
        arg_lines -= 1;
        let arg = next_part(&mut iter);
        println!("ARG: {}", arg);
        let resp_s_arg = match cmd_str {
            "ECHO" => format!("+{}", arg.to_owned()),
            "GET" => {
                let val = map_store.get(&arg).expect("NO VALUE");
                let mut expired_val = false;
                if let Some(exp) = exp_store.get(&arg) {
                    expired_val = *exp < time_now();
                }
                let res = if expired_val {
                    format!("$-1")
                } else {
                    format!("+{}", val.to_owned())
                };
                res
            }
            "SET" => {
                let val = next_part(&mut iter);
                println!("VAL: {}", val);
                arg_lines -= 1;
                let key = arg.clone();
                map_store.insert(arg, val);

                let has_more = arg_lines > 0;
                if has_more {
                    let _ = next_part(&mut iter);
                    let exp_in_ms = next_part(&mut iter)
                        .parse::<u64>()
                        .expect("Expiry not found");
                    println!("EXPIRY: {}", exp_in_ms);
                    arg_lines -= 1;
                    arg_lines -= 1;
                    let next_inst = time_now() + exp_in_ms;
                    exp_store.insert(key, next_inst);
                }
                "+OK".to_owned()
            }
            _ => "+OK".to_owned(),
        };
        let res_str = format!("{}\r\n", resp_s_arg);
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
                let mut exp_map: HashMap<String, u64> = HashMap::new();
                let mut val_map: HashMap<String, String> = HashMap::new();
                while is_connection_open(&s) {
                    let res = handle_stream(&mut s, &mut val_map, &mut exp_map);
                    if res == None {
                        break;
                    };
                }
            });
        }
    }
}
