use flate2::{write::GzEncoder, Compression};
use lib::io::MultiWriter;
use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_tcp_request(mut stream: TcpStream) -> io::Result<()> {
    let req = read_request_line(&mut stream)?;
    handle_http_request(&req, stream)
}

fn read_request_line(req: &mut TcpStream) -> io::Result<String> {
    let mut line_buf = Vec::<u8>::new();
    loop {
        let mut ch_buf = vec![0u8];
        req.read_exact(&mut ch_buf)?;
        if let Some('\n') = ch_buf.first().map(|b| char::from(*b)) {
            break;
        } else {
            line_buf.append(&mut ch_buf);
        }
    }
    Ok(String::from_utf8(line_buf).expect("invalid bytes as UTF-8"))
}

fn handle_http_request(req: &str, res: TcpStream) -> io::Result<()> {
    if let &["GET", "/", ..] = req.split_whitespace().collect::<Vec<_>>().as_slice() {
        get_root(res)?;
    } else {
        eprintln!("cannot accept the following request: {}", req);
    }
    Ok(())
}

fn get_root(mut res: TcpStream) -> io::Result<()> {
    let body = r#"{ "Hello": "World" }"#;

    writeln!(res, "HTTP/1.1 200 OK")?;
    writeln!(res, "Content-Encoding: gzip")?;
    writeln!(res, "Content-Type: application/json")?;
    writeln!(res)?;

    let gzip_encoder = GzEncoder::new(res, Compression::default());
    let mut response_writer =
        MultiWriter::new(vec![Box::new(io::stdout()), Box::new(gzip_encoder)]);

    write!(response_writer, "{}", body)?;

    writeln!(io::stdout())?; // adds a new line to logs

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for res_stream in listener.incoming() {
        let stream = res_stream?;
        handle_tcp_request(stream)?;
    }
    Ok(())
}
