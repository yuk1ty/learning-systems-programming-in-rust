use std::fs::File;
use std::io;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

const ZIP_FILE_NAME: &str = "file.zip";

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let content = std::fs::read(ZIP_FILE_NAME)?;

    writeln!(stream, "HTTP/1.1 200 OK")?;
    writeln!(stream, "Content-Type: application/zip")?;
    writeln!(stream, "Content-Disposition: attachment")?;
    writeln!(stream, "Content-Length: {}", content.len())?;
    writeln!(stream)?;

    stream.write_all(&content)?;

    Ok(())
}

fn create_zip_file() -> io::Result<()> {
    let file = File::create(ZIP_FILE_NAME)?;
    let mut zip = zip::ZipWriter::new(file);
    zip.start_file("file.txt", zip::write::FileOptions::default())?;
    zip.write_all(b"zipzipzipzipzipzipzipzipzipzipzipzipzipzipzipzip")?;

    Ok(())
}

fn main() -> io::Result<()> {
    create_zip_file()?;

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
