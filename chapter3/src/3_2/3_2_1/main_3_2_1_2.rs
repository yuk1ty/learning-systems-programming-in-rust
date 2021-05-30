/**
 *
 * chapter 3_2_1_2
 * io.read
 *
 */
use std::io::Read;

pub fn main() -> std::io::Result<()> {
    let mut r: &[u8] = b"abcdefg";
    let mut buffer = [0; 4];
    r.read_exact(&mut buffer)?;
    println!("{:?}", &buffer[..4]);
    Ok(())
}
