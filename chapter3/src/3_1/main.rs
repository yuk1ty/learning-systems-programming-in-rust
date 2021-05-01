mod bytes_read;

use std::{fs::File, io};

use bytes_read::BytesRead;

/// Reads up to 1024 bytes from `r`.
fn read_1024bytes<R: io::Read>(r: &mut R) -> io::Result<BytesRead> {
    let mut buf = [0u8; 1024];
    let len = r.read(&mut buf)?;
    Ok(BytesRead::new(buf.into(), len))
}

fn main() -> io::Result<()> {
    let mut f = File::open("Cargo.toml")?;

    let bytes_read = read_1024bytes(&mut f)?;
    println!("{}", bytes_read);

    Ok(())
}
