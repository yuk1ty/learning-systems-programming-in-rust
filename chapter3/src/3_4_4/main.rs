use std::io::{Cursor, Read, Seek, SeekFrom};

struct StringReader {
    inner: BytesReader,
}

impl StringReader {
    pub fn new(s: &str) -> Self {
        Self {
            inner: Cursor::new(s.as_bytes().to_vec()),
        }
    }
}

impl Read for StringReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Seek for StringReader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }
}

type BytesReader = Cursor<Vec<u8>>;

fn main() {
    // BufferはRustではVec<u8>とStringが兼ねていると思っています。

    let b_reader1 = BytesReader::new(vec![0x10, 0x20, 0x30]);
    let b_reader2 = BytesReader::new("文字列をバイト配列にキャストして設定".as_bytes().to_vec());

    let s_reader = StringReader::new("Readerの出力内容は文字列で渡す");
}
