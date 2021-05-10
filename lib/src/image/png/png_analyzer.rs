use std::io::Read;

use super::{PngChunks, PNG_FILE_SIGNATURE};

/// Reads PNG file signature and chunks.
///
/// # Examples
///
/// ```no_run
/// use lib::image::png::PngAnalyzer;
/// use std::fs::File;
///
/// let png = File::open("/path/to/file.png").unwrap();
/// let analyzer = PngAnalyzer::new(png);
/// for chunk in analyzer.chunks() {
///     println!("{}", chunk);
/// }
/// ```
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
        assert_eq!(sig, PNG_FILE_SIGNATURE, "wrong PNG file signature");
    }
}
