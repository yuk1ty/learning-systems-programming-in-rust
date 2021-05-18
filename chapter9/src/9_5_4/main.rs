use std::env;
use std::path::{Path, PathBuf};
#[derive(Debug)]
enum PathError {
    IoError(std::io::Error),
    StripPrefixError(std::path::StripPrefixError),
}

impl From<std::io::Error> for PathError {
    fn from(error: std::io::Error) -> Self {
        PathError::IoError(error)
    }
}

impl From<std::path::StripPrefixError> for PathError {
    fn from(error: std::path::StripPrefixError) -> Self {
        PathError::StripPrefixError(error)
    }
}

fn main() -> Result<(), PathError> {
    // パスをそのままクリーンにする
    let path = PathBuf::from("./chapter9/src/../src/9_5_4/main.rs").canonicalize()?;
    let current_dir = env::current_dir()?;
    let clean_path = path.strip_prefix(current_dir)?;
    println!("{:?}", clean_path);

    // パスを絶対パスに整形
    let abs_path = Path::new("./chapter9/src/9_5_4/main.rs").canonicalize()?;
    println!("{:?}", abs_path);

    // パスを相対パスに整形
    let abs_path = Path::new("/usr/local/go/bin/go");
    let rel_path = abs_path.strip_prefix("/usr/local/go")?;
    println!("{:?}", rel_path);

    Ok(())
}
