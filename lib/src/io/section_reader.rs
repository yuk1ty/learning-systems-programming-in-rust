use std::io::{self, Read, Seek, SeekFrom};

/// Reads only limited section (bytes window) of given reader.
/// Useful to read part of a binary file, for example.
///
/// # Examples
///
/// ```
/// use lib::{env::temp_file, io::SectionReader};
/// use std::{fs::OpenOptions, io::{self, Read, Write}};
///
/// fn main() -> io::Result<()> {
///     let mut f = OpenOptions::new()
///         .create(true)
///         .write(true)
///         .read(true)
///         .open(temp_file())?;
///     writeln!(f, "Example of io.SectionReader")?;
///
///     let mut reader = SectionReader::new(f, 14, 7)?;
///     let mut s = String::new();
///     reader.read_to_string(&mut s)?;
///     assert_eq!(&s, "Section");
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct SectionReader<R>
where
    R: Read + Seek,
{
    seeked_reader: R,
    reads_upto: usize,
}

impl<R> SectionReader<R>
where
    R: Read + Seek,
{
    /// Constructs new SectionReader that reads bytes
    /// in range `[offset_byte, min(offset_byte + n_byte, EOF)]` from `reader`.
    pub fn new(mut reader: R, offset_byte: u64, n_byte: usize) -> io::Result<Self> {
        reader.seek(SeekFrom::Start(offset_byte))?;
        Ok(Self {
            seeked_reader: reader,
            reads_upto: n_byte,
        })
    }
}

impl<R> Read for SectionReader<R>
where
    R: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.reads_upto == 0 {
            Ok(0) // pseudo EOF
        } else {
            let read_byes = self.seeked_reader.read(buf)?;
            if read_byes == 0 {
                // got EOF
                Ok(0)
            } else if read_byes <= self.reads_upto {
                self.reads_upto -= read_byes;
                Ok(read_byes)
            } else {
                // actually read `read_bytes` but pretend to did just `reads_upto` bytes.
                let ret = self.reads_upto;
                self.reads_upto = 0;
                Ok(ret)
            }
        }
    }
}
