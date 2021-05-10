//! Toolkit for analyzing / creating PNG format.
//!
//! Intended to be used from chapter3's problems.
//!
//! # Examples
//!
//! ```no_run
//! use lib::image::png::PngAnalyzer;
//! use std::fs::File;
//!
//! let png = File::open("/path/to/file.png").unwrap();
//! let analyzer = PngAnalyzer::new(png);
//! for chunk in analyzer.chunks() {
//!     println!("{}", chunk);
//! }
//! ```

mod png_analyzer;
mod png_chunks;

pub use png_analyzer::PngAnalyzer;
pub use png_chunks::{PngChunk, PngChunkType, PngChunks};
