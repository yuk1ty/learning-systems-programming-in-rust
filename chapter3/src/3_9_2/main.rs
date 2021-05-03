use std::fs::File;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let bytes = rand::thread_rng().gen::<[u8; 1024]>();

    let mut file = File::create("random.txt")?;
    file.write_all(&bytes)?;

    Ok(())
}
