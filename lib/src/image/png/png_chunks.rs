mod png_chunk;

use std::io::Read;

pub use png_chunk::{PngChunk, PngChunkType};

/// Iterator of PngChunk.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct PngChunks<R>
where
    R: Read,
{
    reader_at_next_chunk: R,
}

impl<R> PngChunks<R>
where
    R: Read,
{
    pub fn new(reader_at_first_chunk: R) -> Self {
        Self {
            reader_at_next_chunk: reader_at_first_chunk,
        }
    }
}

impl<R> Iterator for PngChunks<R>
where
    R: Read,
{
    type Item = PngChunk;

    fn next(&mut self) -> Option<PngChunk> {
        PngChunk::from_reader(&mut self.reader_at_next_chunk)
            .expect("IO error while reading PNG chunks")
    }
}
