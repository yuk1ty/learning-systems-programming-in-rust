use std::io::{Read, Write};

pub struct TeeReader<R, W>
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
