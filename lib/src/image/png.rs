//! Toolkit for analyzing / creating PNG format.
//!
//! Intended to be used from chapter3's problems.

mod png_analyzer;
mod png_chunks;
mod png_creator;

pub use png_analyzer::PngAnalyzer;
pub use png_chunks::{PngChunk, PngChunkType, PngChunks};
pub use png_creator::PngCreator;

const PNG_FILE_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
