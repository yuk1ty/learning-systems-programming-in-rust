use std::{io, path::Path};

use crate::linux::{self, arch::x86_64::syscall};

use super::Fd;

const O_RDONLY: i32 = 0;

/// `open(2)` システムコールで、第2引数 `flags` を `O_RDONLY` にしたもの。
pub(crate) fn open_readonly<P: AsRef<Path>>(path: P) -> linux::Result<Fd> {
    let path_ptr = path
        .as_ref()
        .to_str()
        .expect("path should be in UTF-8")
        .as_ptr();

    let raw_fd = syscall::open(path_ptr, O_RDONLY);
    if raw_fd == -1 {
        // open(2) がエラーを返した場合。errnoを返す。
        Err(linux::Error::from(io::Error::last_os_error()))
    } else {
        Ok(Fd::new(raw_fd))
    }
}
