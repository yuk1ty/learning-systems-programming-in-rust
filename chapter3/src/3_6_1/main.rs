use std::io::{BufRead, BufReader, Result};

const SOURCE: &str = r#"1行目
2行目
3行目
"#;

/// 終端文字を気にしながら1行ずつ読み込むパターン。
fn with_eof() -> Result<()> {
    let mut reader = BufReader::new(SOURCE.as_bytes());

    loop {
        let mut buf = String::new();
        // `read_line` は `EOF` を検知してもエラーにすることはなく、
        // `Ok(0)` が返ってきた時点で `EOF` を検知したことになる。
        let num_bytes = reader.read_line(&mut buf)?;
        print!("{}", buf);

        if num_bytes == 0 {
            break;
        }
    }

    Ok(())
}

/// 終端を気にせずもっと短く書きたい場合。
fn without_eof() -> Result<()> {
    let reader = BufReader::new(SOURCE.as_bytes());
    let lines = reader.lines();

    for line in lines {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

fn main() -> Result<()> {
    with_eof()?;
    without_eof()?;

    Ok(())
}
