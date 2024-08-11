use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::message::vec_w_bulk_strs;

fn read_respons(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 100]; // Create a buffer to store the response
    let bytes_read = stream.read(&mut buffer).unwrap(); // Read the response into the buffer
    String::from_utf8_lossy(&buffer[..bytes_read]).to_string()
}
pub fn handshake(mut stream: &mut TcpStream, port: u16) {
    let resp_s = vec_w_bulk_strs(&vec!["PING"]);
    stream
        .write(resp_s.as_bytes())
        .expect("handshake failed - PING");

    println!("PING response: {}", read_respons(&mut stream));

    let repl_conf_port_str = vec_w_bulk_strs(&vec![
        "REPLCONF",
        "listening-port",
        format!("{}", port).as_str(),
    ]);
    stream
        .write(repl_conf_port_str.as_bytes())
        .expect("handshake failed - REPLCONF listening-port");
    println!("REPLCONF port response: {}", read_respons(&mut stream));

    let repl_conf_capa_str = vec_w_bulk_strs(&vec!["REPLCONF", "capa", "psync2"]);
    stream
        .write(repl_conf_capa_str.as_bytes())
        .expect("handshake failed - REPLCONF capa");
    println!("REPLCONF capa response: {}", read_respons(&mut stream));

    let psync_str = vec_w_bulk_strs(&vec!["PSYNC", "?", "-1"]);
    stream
        .write(psync_str.as_bytes())
        .expect("handshake failed - PSYNC capa");
    println!("REPLCONF psync response: {}", read_respons(&mut stream));
}
