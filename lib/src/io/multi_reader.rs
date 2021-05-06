use std::io::Read;

/// Takes multiple `std::io::Read` at once.
/// This is inspired by `io.MultiReader` in Go.
///
/// # Example
///
/// ```
/// use std::{
///     io::{copy, stdout, Read},
///     usize,
/// };
/// use lib::io::MultiReader;
///
/// fn main() -> std::io::Result<()> {
///     let header = "---- HEADER ----\n".as_bytes();
///     let content = "Example of MultiReader\n".as_bytes();
///     let footer = "---- FOOTER ----\n".as_bytes();
///     let mut multi_reader = MultiReader::new(vec![header, content, footer]);
///     copy(&mut multi_reader, &mut stdout())?;
///     Ok(())
/// }
/// ```
pub struct MultiReader<R: Read> {
    readers: Vec<R>,
    /// Points to where we read right now.
    pos: usize,
}

impl<R: Read> MultiReader<R> {
    /// Creates `MultiReader`. `pos` is set to 0 by default.
    pub fn new(readers: Vec<R>) -> Self {
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
            self.pos += 1;
        }
    }
}
