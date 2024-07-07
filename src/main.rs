use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    fn handle_stream(mut stream: TcpStream) -> Result<usize> {
        let mut buf: [u8; 512] = [0; 512];
        stream.read(&mut buf).unwrap();

        let cmd_str = std::str::from_utf8(&buf).unwrap();
        println!("RECEIVED command: `{}`", cmd_str);
        let resp_str: &str = match cmd_str {
            "PING" => "+PONG\r\n",
            _ => "-ERR unknown command\r\n",
        };

        println!("RESPONDED: `{}`", resp_str);
        let cmd_resp = resp_str.as_bytes();
        let written = stream.write(cmd_resp)?;
        return Ok(written);
    }

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                // let num_written = handle_stream(stream).unwrap();
                // println!("{} bytes written", num_written);
                stream.write("+PONG\r\n".as_bytes()).unwrap();
                continue;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
