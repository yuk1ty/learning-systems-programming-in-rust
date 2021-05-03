/**
 *
 * chapter 3_2_1_1
 * io.read
 *
 */
use std::io::Read;

pub fn main() -> std::io::Result<()> {
    let mut r: &[u8] = &b"abcdefg".repeat(5); //make data
    let mut buffer: Vec<u8> = vec![];
    let size = r.read_to_end(&mut buffer)?; 
    println!("{:?}", &buffer[..size]);
    Ok(())
}
