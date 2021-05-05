use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("test.txt")?;
    file.write_all(b"std::fs::File example\n")?;
    Ok(())
}
