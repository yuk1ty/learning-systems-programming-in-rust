use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    let remote_addr = "localhost:8888";
    let socket = UdpSocket::bind("localhost:0")?;
    println!("Sending to the server");

    socket.send_to("Hello from client".as_bytes(), remote_addr)?;
    let mut buf = [0u8; 1500];
    let (length, _) = socket.recv_from(&mut buf)?;

    println!(
        "Received: {}",
        str::from_utf8(&buf[..length]).expect("buf is not utf-8")
    );

    Ok(())
}
