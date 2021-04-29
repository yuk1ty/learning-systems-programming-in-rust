use std::net::{TcpStream, TcpListener};
use std::io::{Write, BufReader, BufRead};

fn get_operation(stream: &mut TcpStream) {
    let body = "HTTP server sample";
    writeln!(stream, "HTTP/1.1 200 OK").unwrap();
    writeln!(stream, "Content-Type: text/html; charset=UTF-8").unwrap();
    writeln!(stream, "Content-Length: {}", body.len()).unwrap();
    writeln!(stream).unwrap();

    writeln!(stream, "{}", body).unwrap();
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream);

    let mut first_line = String::new();
    if let Err(err) = reader.read_line(&mut first_line) {
        panic!("error during receive a line: {}", err);
    }

    let mut params = first_line.split_whitespace();
    let method = params.next();
    let path = params.next();
    match (method, path) {
        (Some("GET"), Some(_)) => {
            get_operation(reader.get_mut());
        }
        _ => panic!("failed to parse"),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream)
            }
            Err(_) => { panic!("connection failed") }
        };
    }
}
