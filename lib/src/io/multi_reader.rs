use std::io::Read;

pub struct MultiReader<R> {
    readers: Vec<R>,
    pos: usize,
}

impl<R: Read> MultiReader<R> {
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
            self.pos = self.pos + 1;
        }
    }
}
