use std::fs;
use std::io::{self, Write};

//ファイル作成/読み込み

// 新規作成
fn open() -> std::io::Result<()> {
    let mut file = fs::File::create("textfile.txt")?;
    file.write_all(b"New file content\n")?;
    Ok(())
}

// 読み込み
fn read() -> std::io::Result<()> {
    let mut file = fs::File::open("textfile.txt")?;
    println!("Read file:");
    io::copy(&mut file, &mut io::stdout())?;
    Ok(())
}

// 追記モード
fn append() -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("textfile.txt")?;
    file.write_all(b"Appened content\n")?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    open()?;
    read()?;
    append()?;
    read()?;
    Ok(())
}
