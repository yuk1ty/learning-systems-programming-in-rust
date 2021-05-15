use lib::binary;
use std::io;

fn main() -> io::Result<()> {
    let mut bytes: &[u8] = &[0x0, 0x0, 0x27, 0x10];
    let n_big: u32 = binary::read(&mut bytes, &binary::Endian::BigEndian)?;
    println!("Big endian: {}", n_big);

    let mut bytes: &[u8] = &[0x0, 0x0, 0x27, 0x10];
    let n_little: u32 = binary::read(&mut bytes, &binary::Endian::LittleEndian)?;
    println!("Little endian: {}", n_little);

    Ok(())
}
