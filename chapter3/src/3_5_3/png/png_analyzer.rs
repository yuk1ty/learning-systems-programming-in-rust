use std::io::Read;

use super::PngChunks;

#[derive(Debug)]
pub struct PngAnalyzer<R>
where
    R: Read,
{
    png_reader: R,
}

impl<R> PngAnalyzer<R>
where
    R: Read,
{
    pub fn new(png_reader: R) -> Self {
        Self { png_reader }
    }

    pub fn chunks(mut self) -> PngChunks<R> {
        self.read_signature();
        PngChunks::new(self.png_reader)
    }

    /// # Panics
    ///
    /// - Leading 8 bytes do not match PNG file signature.
    /// - Got any io::Error internally.
    fn read_signature(&mut self) {
        let mut sig = [0u8; 8];
        self.png_reader.read_exact(&mut sig).unwrap();
        assert_eq!(
            sig,
            [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
            "wrong PNG file signature"
        );
    }
}
