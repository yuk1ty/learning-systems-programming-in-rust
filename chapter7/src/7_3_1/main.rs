//! Timer *server* (*sender* would be better understood).

use std::net::UdpSocket;

fn main() {
    let res_addr = "0.0.0.0:0";
    let socket = UdpSocket::bind(res_addr)
        .unwrap_or_else(|e| panic!("failed to bind {} : {:?}", res_addr, e));

    let req_addr = "224.0.0.1:9999";
    socket
        .connect(req_addr)
        .unwrap_or_else(|e| panic!("failed to connect to {} : {:?}", req_addr, e));

    socket.send(&[0, 1, 2]).expect("failed to send a message");
}
