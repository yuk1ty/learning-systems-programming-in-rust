use std::io::{self, Read, Seek};

use super::SectionReader;

/// Reads only limited bytes of given reader from start.
/// Useful to read header section of a binary file, for example.
///
/// # Examples
///
/// ```
/// use lib::{env::temp_file, io::LimitedReader};
/// use std::{fs::OpenOptions, io::{self, Read, Write}};
///
/// fn main() -> io::Result<()> {
///     let mut f = OpenOptions::new()
///         .create(true)
///         .write(true)
///         .read(true)
///         .open(temp_file())?;
///     writeln!(f, "Example of io.LimitedReader")?;
///
///     let mut reader = LimitedReader::new(f, 7)?;
///     let mut s = String::new();
///     reader.read_to_string(&mut s)?;
///     assert_eq!(&s, "Example");
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct LimitedReader<R>(SectionReader<R>)
where
    R: Read + Seek;

impl<R> LimitedReader<R>
where
    R: Read + Seek,
{
    /// Constructs new LimitedReader that reads bytes
    /// from start to `min(n_byte, EOF)` from `reader`.
    pub fn new(reader: R, n_byte: usize) -> io::Result<Self> {
        let section_reader = SectionReader::new(reader, 0, n_byte)?;
        Ok(Self(section_reader))
    }
}

impl<R> Read for LimitedReader<R>
where
    R: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}
