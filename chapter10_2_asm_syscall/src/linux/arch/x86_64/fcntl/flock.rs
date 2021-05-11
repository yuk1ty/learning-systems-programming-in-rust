use std::io;

use crate::linux::{self, arch::x86_64::syscall};

use super::fd::Fd;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum FlockOperation {
    LockExclusive,
    Unlock,
}

impl FlockOperation {
    fn to_c_int(&self) -> i32 {
        match self {
            FlockOperation::LockExclusive => 2,
            FlockOperation::Unlock => 8,
        }
    }
}

/// `flock(2)` に対応。
pub(crate) fn flock(fd: &Fd, operation: FlockOperation) -> linux::Result<()> {
    // TODO mutex

    println!("fd={:?}", fd);

    let ret = syscall::flock(fd.to_i32(), operation.to_c_int());
    if ret == -1 {
        // flock(2) がエラーを返した場合。errnoを返す。
        Err(linux::Error::from(io::Error::last_os_error()))
    } else {
        debug_assert_eq!(ret, 0);
        Ok(())
    }
}
