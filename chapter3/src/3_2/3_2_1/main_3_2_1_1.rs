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
    let size = match r.read_to_end(&mut buffer) {
        Ok(size) => size,
        Err(err) => {
            panic!("ERROR => {}", err)
        }
    };
    println!("{:?}", &buffer[..size]);
    Ok(())
}
