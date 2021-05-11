/// ファイルディスクリプタ
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct Fd(i32);

impl Fd {
    pub(crate) fn new(raw_fd: i32) -> Self {
        Self(raw_fd)
    }

    pub(crate) fn to_i32(&self) -> i32 {
        self.0
    }
}

// TODO Drop で close
