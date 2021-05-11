use std::path::Path;

use crate::linux;

use super::Fd;

/// `open(2)` システムコールで、第2引数 `flags` を `O_RDONLY` にしたもの。
pub(crate) fn open_readonly<P: AsRef<Path>>(path: P) -> linux::Result<Fd> {
    todo!()
}
