use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    println!("The server is running at localhost:8888");
    let socket = UdpSocket::bind("localhost:8888")?;

    loop {
        let mut buf = [0u8; 1500];
        let (length, remote_addr) = socket.recv_from(&mut buf)?;

        println!(
            "Received from {}: {}",
            remote_addr,
            str::from_utf8(&buf[..length]).expect("buf is not utf-8")
        );

        socket.send_to("Hello from Server".as_bytes(), remote_addr)?;
    }
}
