mod endian;
mod fixed_len_bytes;

pub use endian::Endian;
pub use fixed_len_bytes::FixedLenBytes;

use std::io::{self, Read};

/// Read bytes of fixed size (`B::len()`) from `reader` in `endian` order.
///
/// # Examples
///
/// ```
/// use lib::binary;
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     let mut bytes: &[u8] = &[0x01, 0x02, 0x03, 0x04];
///     let n_big: u32 = binary::read(&mut bytes, &binary::Endian::BigEndian)?;
///     assert_eq!(n_big, 0x01020304);
///
///     let mut bytes: &[u8] = &[0x01, 0x02, 0x03, 0x04];
///     // reads only first 2 bytes
///     let n_little: u16 = binary::read(&mut bytes, &binary::Endian::LittleEndian)?;
///     assert_eq!(n_little, 0x0201);
///
///     Ok(())
/// }
/// ```
pub fn read<R, B>(reader: &mut R, endian: &Endian) -> io::Result<B>
where
    R: Read,
    B: FixedLenBytes,
{
    let mut buf = Vec::<u8>::new();
    buf.resize(B::len(), 0);

    reader.read_exact(&mut buf)?;

    match endian {
        Endian::BigEndian => {}
        Endian::LittleEndian => {
            buf.reverse();
        }
    }

    let bytes = B::new(&buf).expect("buf has wrong length");
    Ok(bytes)
}
