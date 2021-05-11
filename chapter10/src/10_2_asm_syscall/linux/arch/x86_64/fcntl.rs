use std::path::Path;

use crate::linux;

/// ファイルディスクリプタ
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct Fd(i32);

/// `open(2)` システムコールで、第2引数 `flags` を `O_RDONLY` にしたもの。
pub(crate) fn open_readonly<P: AsRef<Path>>(path: P) -> linux::Result<Fd> {
    todo!()
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum FlockOperation {
    LockExclusive,
    Unlock,
}

/// `flock(2)` に対応。
pub(crate) fn flock(fd: &Fd, operation: FlockOperation) -> linux::Result<()> {
    todo!()
}
