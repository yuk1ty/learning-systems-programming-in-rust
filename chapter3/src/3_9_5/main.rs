use std::fs::File;
use std::io::{Read, Result, Write};

/// golangのio.CopyNは次のようなシグニチャになっており、dst(コピー先)を第1引数、src(コピー元)を第2引数としている
/// ```golang
/// func CopyN(dst Writer, src Reader, n int64) (written int64, err error)
/// ```
/// Rustのio::copyを見ると、reader(コピー元)が第1,, writer(コピー先)が第２となっているのでそちらに合わせた
pub fn copy_n<R, W>(reader: &mut R, writer: &mut W, n: usize) -> Result<u64>
where
    R: Read,
    W: Write,
{
    let mut buffer = vec![0; n];
    reader.read_exact(&mut buffer)?;

    writer.write_all(&buffer)?;

    Ok(0)
}

fn main() -> Result<()> {
    let mut file = File::open("test.txt").expect(&format!(
        "動作には4バイト以上の {}/test.txt が必要です",
        std::env::current_dir()?.display()
    ));
    let mut new_file = File::create("new_test.txt")?;

    copy_n(&mut file, &mut new_file, 4)?;

    Ok(())
}
