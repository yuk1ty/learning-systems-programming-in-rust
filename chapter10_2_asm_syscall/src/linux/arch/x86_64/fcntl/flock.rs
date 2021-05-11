use crate::linux;

use super::fd::Fd;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum FlockOperation {
    LockExclusive,
    Unlock,
}

/// `flock(2)` に対応。
pub(crate) fn flock(fd: &Fd, operation: FlockOperation) -> linux::Result<()> {
    todo!()
}
