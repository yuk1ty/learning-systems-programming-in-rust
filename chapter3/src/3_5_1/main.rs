use std::{
    fs::OpenOptions,
    io::{self, Write},
};

use lib::{env::temp_file, io::SectionReader};

fn main() -> io::Result<()> {
    // File implements both Read & Seek
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(temp_file())?;
    writeln!(f, "Example of io.SectionReader")?;

    let mut reader = SectionReader::new(f, 14, 7)?;
    io::copy(&mut reader, &mut io::stdout())?;

    Ok(())
}
