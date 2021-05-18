use lib::path::*;
use std::path::Path;

fn main() -> Result<(), PathError> {
    //存在しないパスを指定した場合、エラーを返す

    // パスをそのままクリーンにする
    let path = Path::new("./chapter9/src/../src/9_5_4/main.rs").clean()?;
    println!("{:?}", path);

    // パスを絶対パスに整形
    let abs_path = Path::new("./chapter9/src/9_5_4/main.rs").canonicalize()?;
    println!("{:?}", abs_path);

    // パスを相対パスに整形
    let abs_path = Path::new("/usr/local/go/bin/go");
    let rel_path = abs_path.strip_prefix("/usr/local/go")?;
    println!("{:?}", rel_path);

    Ok(())
}
