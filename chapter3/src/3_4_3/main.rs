use anyhow::Result;
use httparse::{parse_headers, Error, Header, Response, Status, EMPTY_HEADER};
use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, BufRead, BufReader, Bytes, Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct HeaderOwn {
    name: String,
    body: Vec<u8>,
}

struct ReadResponse<R: Read + BufRead> {
    headers: Vec<HeaderOwn>,
    body: R,
}

impl<R: Read + BufRead> ReadResponse<R> {
    fn new(mut reader: R) -> Result<Self> {
        let mut buf = vec![];
        let headers = loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            buf.extend_from_slice(&line.as_bytes());

            let mut headers = [EMPTY_HEADER; 30];
            let mut resp = Response::new(&mut headers);
            let status = resp.parse(&buf)?;
            if status.is_complete() {
                break resp
                    .headers
                    .iter()
                    .map(|h| HeaderOwn {
                        name: h.name.to_string(),
                        body: h.value.to_vec(),
                    })
                    .collect();
            }
        };

        Ok(Self {
            headers,
            body: reader,
        })
    }
}

fn main() {
    let mut conn = TcpStream::connect("example.com:80").unwrap();

    conn.write("GET / HTTP/1.0\r\nHost: example.com\r\n\r\n".as_bytes())
        .unwrap();
    let mut buf_reader = BufReader::new(conn);
    let mut res = ReadResponse::new(&mut buf_reader).unwrap();

    println!("{:?}", res.headers);

    std::io::copy(&mut res.body, &mut stdout());
}
