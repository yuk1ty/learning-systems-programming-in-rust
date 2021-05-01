use std::fs::File;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let mut bytes = [0; 1024];
    for byte in bytes.iter_mut() {
        *byte = rand::random()
    }

    let mut file = File::create("random.txt")?;
    file.write_all(&bytes)?;

    Ok(())
}
