use lib::io::TeeReader;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // TeeReader を使用した場合の例。
    let mut buf = Vec::new();
    let reader = "Example of TeeReader".as_bytes();
    let mut tee_reader = TeeReader::new(reader, &mut buf);
    // データを読み捨てる。
    let _ = tee_reader.read_to_end(&mut Vec::new())?;

    println!(
        "{}",
        // けどバッファには残っている。
        String::from_utf8(buf)
            .expect("UTF-8 形式でない文字列の可能性があります。UTF-8 にしてください。")
    );

    Ok(())
}
