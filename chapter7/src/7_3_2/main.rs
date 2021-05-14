//! Timer *client* (*receiver* would be better understood).

use std::{
    net::{Ipv4Addr, UdpSocket},
    str::FromStr,
};

fn main() {
    let recv_host = "224.0.0.1";
    let recv_addr = format!("{}:9999", recv_host);

    let socket = UdpSocket::bind(&recv_addr)
        .unwrap_or_else(|e| panic!("failed to bind {} : {:?}", recv_addr, e));

    socket
        .join_multicast_v4(
            &Ipv4Addr::from_str(recv_host).unwrap(),
            &Ipv4Addr::UNSPECIFIED,
        )
        .expect("failed to join multicast group");

    let mut buf = [0u8; 100];

    println!("waiting for tick...");
    socket.recv_from(&mut buf).expect("Didn't receive data");
    println!("received!");

    let tick_s = String::from_utf8(buf.to_vec()).expect("tick data should be written in UTF-8");
    println!("{}", tick_s);
}
