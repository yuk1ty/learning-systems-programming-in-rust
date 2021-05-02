use std::io::{self, Write};

pub(super) struct MultiWriter(Vec<Box<dyn Write>>);

impl MultiWriter {
    pub(super) fn new(writers: Vec<Box<dyn Write>>) -> Self {
        Self(writers)
    }
}

impl Write for MultiWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for w in &mut self.0 {
            // Using `Write::write` is more straightforward but
            // it sometimes (especially when buf is large) writes only part of buf.
            // To align bytes written for all writers (self.0), here uses `Write::write_all`.
            w.write_all(buf)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        for w in &mut self.0 {
            w.flush()?;
        }
        Ok(())
    }
}
