//! Timer *server* (*sender* would be better understood).

use std::{
    net::UdpSocket,
    thread,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Local};

fn main() {
    let res_addr = "0.0.0.0:0";
    let socket = UdpSocket::bind(res_addr)
        .unwrap_or_else(|e| panic!("failed to bind {} : {:?}", res_addr, e));

    let req_addr = "224.0.0.1:9999";
    socket
        .connect(req_addr)
        .unwrap_or_else(|e| panic!("failed to connect to {} : {:?}", req_addr, e));

    println!("Start tick server requesting to {}", req_addr);

    // sends current time per 10 secs.
    loop {
        let now = SystemTime::now();
        let datetime: DateTime<Local> = now.into();
        let now_s = format!("{}", datetime.format("%d/%m/%Y %T"));

        println!("Tick: {}", &now_s);
        socket
            .send(now_s.as_bytes())
            .expect("failed to send a message");
        thread::sleep(Duration::from_secs(10));
    }
}
