use std::io::{copy, stdout, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("ascii.jp:80")?;
    let _ = stream.write(b"GET / HTTP/1.0\r\nHost: ascii.jp\r\n\r\n")?;
    copy(&mut stream, &mut stdout())?;
    Ok(())
}
