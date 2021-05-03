use std::io::{self, Write};

/// Takes 0, 1, or more `std::io::Write`s and writes to them at a time.
/// `MultiWriter` implements `std::io::Write` trait.
///
/// Inspired by Go's MultiWriter.
///
/// # Examples
///
/// ```
/// use lib::io::MultiWriter;
/// use std::{fs::File, io::{self, Write}};
///
/// fn main() -> io::Result<()> {
///     let stdout = io::stdout();
///     let f = File::create("a.txt")?;
///
///     let mut mw = MultiWriter::new(vec![Box::new(stdout), Box::new(f)]);
///
///     mw.write("xxx".as_bytes())?;
///     mw.flush()?;
///
///     Ok(())
/// }
/// ```
pub struct MultiWriter(Vec<Box<dyn Write>>);

impl MultiWriter {
    /// Constructor.
    pub fn new(writers: Vec<Box<dyn Write>>) -> Self {
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
