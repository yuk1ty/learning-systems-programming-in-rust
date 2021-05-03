use rand::Rng;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let mut bytes = [0u8; 1024];
    rand::thread_rng().fill(&mut bytes);

    let mut file = File::create("random.txt")?;
    file.write_all(&bytes)?;

    Ok(())
}
