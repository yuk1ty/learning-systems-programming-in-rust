use std::fmt::Display;

/// PNG's chunk types.
/// "IHDR", "sRGB", for example.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct PngChunkType([u8; 4]);

impl PngChunkType {
    pub fn is_text(&self) -> bool {
        self.0 == "tEXt".as_bytes()
    }

    pub(super) fn new(data: [u8; 4]) -> Self {
        Self(data)
    }

    pub(super) fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Display for PngChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = String::from_utf8(self.0.to_vec())
            .expect("PNG's chunk types are in 4-byte printable ASCII.");
        write!(f, "{}", type_str)
    }
}
