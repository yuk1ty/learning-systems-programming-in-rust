use std::net::TcpStream;
use std::io::{Write, Read, BufReader};

fn main() {
    let mut conn = TcpStream::connect("ascii.jp:80").unwrap();

    conn.write("GET / HTTP/1.0\r\nHost: ascii.jp\r\n\r\n".as_bytes()).unwrap();

    BufReader::new(conn);
    conn.read(&mut buf);
}
