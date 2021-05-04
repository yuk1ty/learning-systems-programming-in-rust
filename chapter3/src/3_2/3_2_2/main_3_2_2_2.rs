/**
 *
 * chapter 3_2_2_2
 * io.copy with size
 *
 */
use std::io::{copy, Read, Result, Write};

pub fn main() -> std::io::Result<()> {
    let mut reader: &[u8] = b"hello!";
    let mut buffer: Vec<u8> = vec![];
    copy_n(&mut reader, &mut buffer, 3)?;
    println!("{:?}", &buffer[..]);
    Ok(())
}

fn copy_n<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W, size: u64) -> Result<u64>
where
    R: Read,
    W: Write,
{
    return copy(&mut reader.take(size), writer);
}
