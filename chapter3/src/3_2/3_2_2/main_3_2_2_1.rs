/**
 *
 * chapter 3_2_2_1
 * io.copy
 *
 */
use std::io::copy;

pub fn main() -> std::io::Result<()> {
    let mut reader: &[u8] = b"hello!";
    let mut buffer: Vec<u8> = vec![];
    copy(&mut reader, &mut buffer)?;
    println!("{:?}", &buffer[..]);
    Ok(())
}
