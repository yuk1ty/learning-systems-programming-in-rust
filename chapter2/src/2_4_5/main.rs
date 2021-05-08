use flate2::write::GzEncoder;
use flate2::Compression;
use lib::io::MultiWriter;
use std::{fs::File, io::BufWriter, io::Write};

fn main() -> std::io::Result<()> {
    let file = File::create("test.txt")?;
    let stdout = std::io::stdout();
    let mut writer = MultiWriter::new(vec![Box::new(file), Box::new(stdout)]);
    writer.write_all(b"example\n")?;

    let file2 = File::create("test.txt.gz")?;
    let mut writer2 = GzEncoder::new(file2, Compression::default());
    writer2.write_all(b"gzip.Writer example\n")?;

    let mut buffer = BufWriter::new(std::io::stdout());
    buffer.write_all(b"bufio.Writer ")?;
    buffer.write_all(b"example\n")?;

    Ok(())
}
