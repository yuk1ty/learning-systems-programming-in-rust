use super::{PngChunk, PNG_FILE_SIGNATURE};

/// Creates a PNG binary.
#[derive(Debug, Default)]
pub struct PngCreator(Vec<PngChunk>);

impl PngCreator {
    pub fn add_chunk(&mut self, chunk: PngChunk) {
        self.0.push(chunk)
    }

    pub fn finalize(self) -> Vec<u8> {
        let mut bin = PNG_FILE_SIGNATURE.to_vec();
        for chunk in self.0 {
            bin.append(&mut chunk.into_vec());
        }
        bin
    }
}
