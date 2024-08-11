use std::{io::Write, net::TcpStream};

use crate::message::{kv_as_bulk_str, vec_as_bulk_str};

pub fn send_ping(stream: &mut TcpStream) {
    let ping_msg = kv_as_bulk_str(&vec!["PING"]);
    let resp_s = vec_as_bulk_str(&vec![ping_msg]) + "\r\n";
    stream
        .write(resp_s.as_bytes())
        .expect("handshake failed - PING");
}
