use lib::path::{ExtendedPath, PathError};
use std::path::{Path, PathBuf};

// パス中のenvを展開した上でcleanする
fn clean2(path: &Path) -> Result<PathBuf, PathError> {
    let path = path.expand_env()?;
    let path = path.clean()?;
    Ok(path)
}

fn main() -> Result<(), PathError> {
    //存在しないパスを指定した場合、エラーを返す

    let path = Path::new("${HOME}/.bashrc");
    println!("{:?}", clean2(path)?);

    let path = Path::new("~/.bashrc");
    println!("{:?}", clean2(path)?);

    Ok(())
}
