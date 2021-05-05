//! Rust には MultiReader と TeeReader は存在しないため、それらを自前で実装して用意しています。

use std::{
    io::{copy, stdout, Read, Write},
    usize,
};

pub struct MultiReader<R> {
    readers: Vec<R>,
    pos: usize,
}

impl<R: Read> MultiReader<R> {
    fn new(readers: Vec<R>) -> Self {
        Self { readers, pos: 0 }
    }
}

impl<R: Read> Read for MultiReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        loop {
            match self.readers.get_mut(self.pos) {
                Some(r) => {
                    let n = r.read(buf)?;
                    if n > 0 {
                        return Ok(n);
                    }
                }
                None => return Ok(0),
            }
            self.pos = self.pos + 1;
        }
    }
}

struct TeeReader<R, W>
where
    R: Read,
    W: Write,
{
    reader: R,
    writer: W,
}

impl<R, W> TeeReader<R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }
}

impl<R, W> Read for TeeReader<R, W>
where
    R: Read,
    W: Write,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.reader.read(buf)?;
        self.writer.write_all(&buf[..n])?;
        Ok(n)
    }
}

fn main() -> std::io::Result<()> {
    let header = "---- HEADER ----\n".as_bytes();
    let content = "Example of MultiReader\n".as_bytes();
    let footer = "---- FOOTER ----\n".as_bytes();
    let mut multi_reader = MultiReader::new(vec![header, content, footer]);
    copy(&mut multi_reader, &mut stdout())?;

    // TeeReader を使用した場合の例。
    let mut buf = Vec::new();
    let reader = "Example of TeeReader".as_bytes();
    let mut tee_reader = TeeReader::new(reader, &mut buf);
    // データを読み捨てる。
    let _ = tee_reader.read_to_end(&mut Vec::new())?;

    println!(
        "{}",
        // けどバッファには残っている。
        String::from_utf8(buf)
            .expect("UTF-8 形式でない文字列の可能性があります。UTF-8 にしてください。")
    );

    Ok(())
}
