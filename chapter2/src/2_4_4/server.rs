use std::net::{TcpStream, TcpListener};
use std::io::{Write, BufReader, BufRead};

fn get_operation(stream: &mut TcpStream) -> std::io::Result<()> {
    let body = "HTTP server sample";
    writeln!(stream, "HTTP/1.1 200 OK")?;
    writeln!(stream, "Content-Type: text/plain; charset=UTF-8")?;
    writeln!(stream, "Content-Length: {}", body.len())?;
    writeln!(stream)?;

    writeln!(stream, "{}", body)?;
    Ok(())
}

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream);

    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    let mut params = first_line.split_whitespace();
    let method = params.next();
    let path = params.next();
    match (method, path) {
        (Some("GET"), Some(_)) => {
            get_operation(reader.get_mut())?;
        }
        _ => panic!("failed to parse"),
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
