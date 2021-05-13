use std::{array::TryFromSliceError, convert::TryInto};

/// Represents data types whose size are fixed.
pub trait FixedLenBytes {
    /// Length in bytes
    fn len() -> usize;

    /// Construct from byte sequence (in big endian).
    ///
    /// # Failures
    ///
    /// `TryFromSliceError` when `buf.len() != Self::len()`.
    fn new(buf: &[u8]) -> Result<Self, TryFromSliceError>
    where
        Self: Sized;
}

impl FixedLenBytes for u8 {
    fn len() -> usize {
        1
    }

    fn new(buf: &[u8]) -> Result<Self, TryFromSliceError>
    where
        Self: Sized,
    {
        let arr = buf.try_into()?;
        Ok(Self::from_be_bytes(arr))
    }
}

impl FixedLenBytes for u16 {
    fn len() -> usize {
        2
    }

    fn new(buf: &[u8]) -> Result<Self, TryFromSliceError>
    where
        Self: Sized,
    {
        let arr = buf.try_into()?;
        Ok(Self::from_be_bytes(arr))
    }
}

impl FixedLenBytes for u32 {
    fn len() -> usize {
        4
    }

    fn new(buf: &[u8]) -> Result<Self, TryFromSliceError>
    where
        Self: Sized,
    {
        let arr = buf.try_into()?;
        Ok(Self::from_be_bytes(arr))
    }
}

impl FixedLenBytes for u64 {
    fn len() -> usize {
        8
    }

    fn new(buf: &[u8]) -> Result<Self, TryFromSliceError>
    where
        Self: Sized,
    {
        let arr = buf.try_into()?;
        Ok(Self::from_be_bytes(arr))
    }
}
