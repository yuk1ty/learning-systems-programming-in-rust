use std::fs::File;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let file = File::create("file.zip")?;

    let mut zip = zip::ZipWriter::new(file);
    zip.start_file("file.txt", zip::write::FileOptions::default())?;
    zip.write_all(b"zipzipzipzipzipzipzipzipzipzipzipzipzipzipzipzip")?;

    Ok(())
}
