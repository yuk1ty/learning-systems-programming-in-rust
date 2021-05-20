use std::{collections::VecDeque, io::Read};

/// Takes multiple `std::io::Read` at once.
/// This is inspired by `io.MultiReader` in Go.
///
/// # Example
///
/// ```
/// use std::io::{copy, stdout};
/// use lib::io::MultiReader;
///
/// fn main() -> std::io::Result<()> {
///     let header = "---- HEADER ----\n".as_bytes();
///     let content = "Example of MultiReader\n".as_bytes();
///     let footer = "---- FOOTER ----\n".as_bytes();
///     let mut multi_reader =
///         MultiReader::new(vec![Box::new(header), Box::new(content), Box::new(footer)]);
///     copy(&mut multi_reader, &mut stdout())?;
///     Ok(())
/// }
/// ```
pub struct MultiReader {
    readers: VecDeque<Box<dyn Read>>,
    /// Points to current element while reading.
    current: Option<Box<dyn Read>>,
}

impl MultiReader {
    /// Constructs `MultiReader`. `current` is set to the first element that is popped out from `VecDeque`.
    pub fn new(readers: Vec<Box<dyn Read>>) -> Self {
        let mut deque: VecDeque<Box<dyn Read>> = readers.into();
        let current = deque.pop_front();
        Self {
            readers: deque,
            current,
        }
    }
}

impl Read for MultiReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Doesn't move to the next one until the first one is completely read.
        // If the processing ends in the middle of reading the first one,
        // it returns the control at that point.
        loop {
            match self.current {
                Some(ref mut r) => {
                    let n = r.read(buf)?;
                    if n > 0 {
                        return Ok(n);
                    }
                }
                None => return Ok(0),
            }
            self.current = self.readers.pop_front();
        }
    }
}
