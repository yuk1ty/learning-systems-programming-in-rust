use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, Write},
};

use lib::{
    env::temp_file,
    io::{LimitedReader, SectionReader},
};

fn create_read_write_temp_file(contents: &str) -> io::Result<File> {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(temp_file())?;
    writeln!(f, "{}", contents)?;
    Ok(f)
}

fn section_reader<R: Read + Seek>(reader: R) -> io::Result<()> {
    let mut section_reader = SectionReader::new(reader, 14, 7)?;
    io::copy(&mut section_reader, &mut io::stdout())?;
    Ok(())
}

fn limited_reader<R: Read + Seek>(reader: R) -> io::Result<()> {
    let mut limited_reader = LimitedReader::new(reader, 16)?;
    io::copy(&mut limited_reader, &mut io::stdout())?;
    Ok(())
}

fn main() -> io::Result<()> {
    // File implements both Read and Seek
    let f1 = create_read_write_temp_file("Example of io.SectionReader")?;
    section_reader(f1)?;

    println!();

    let f2 = create_read_write_temp_file("Example of io.LimitedReader")?;
    limited_reader(f2)?;

    Ok(())
}
