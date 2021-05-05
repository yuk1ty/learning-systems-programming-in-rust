/**
 *
 * chapter 3_2_2_3
 * copy with buffer
 *
 */
use std::io::{Read, Result, Write};

pub fn main() -> std::io::Result<()> {
    let mut read: &[u8] = &b"abcdefg".repeat(500 * 1024); //358.4KB Source
    let mut dst: Vec<u8> = vec![];
    const BUFFER_SIZE: usize = 8 * 1024;
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE]; //8KB Buffer
    let result = copy_buffer(&mut read, &mut dst, &mut buffer)?;
    println!("{} bytes written.", result);
    Ok(())
}

fn copy_buffer<R: ?Sized, W: ?Sized>(
    reader: &mut R,
    writer: &mut W,
    buffer: &mut [u8],
) -> Result<usize>
where
    R: Read,
    W: Write,
{
    let mut wsize: usize;
    let mut written: usize = 0;
    loop {
        match reader.read(buffer)? {
            0 => break,
            rsize => {
                wsize = writer.write(&buffer[..rsize])?;
                written += wsize;
            }
        }
    }
    writer.flush()?;
    Ok(written)
}
