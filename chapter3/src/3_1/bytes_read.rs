use std::fmt::Display;

#[derive(Debug)]
pub(super) struct BytesRead {
    bytes: Vec<u8>,
    len: usize,
}

impl BytesRead {
    pub(super) fn new(bytes: Vec<u8>, len: usize) -> Self {
        Self { bytes, len }
    }
}

impl Display for BytesRead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bytes read:\n{:02x?}", self.len, self.bytes)
    }
}
