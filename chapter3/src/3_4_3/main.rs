use anyhow::Result;
use httparse::{Response, EMPTY_HEADER};
use std::io::{stdout, BufRead, BufReader, Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct HeaderOwn {
    name: String,
    value: String,
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
                    .map(|h| Ok(HeaderOwn {
                        name: h.name.to_string(),
                        value: String::from_utf8(h.value.to_vec())?,
                    }))
                    .collect::<Result<_>>()?;
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

    std::io::copy(&mut res.body, &mut stdout()).unwrap();
}
