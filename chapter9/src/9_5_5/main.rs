use lib::path::{ExtendedPath, PathError};
use std::path::{Path, PathBuf};

fn clean2(path: &Path) -> Result<PathBuf, PathError> {
    let path = path.expand_env()?;
    let path = path.clean()?;
    Ok(path)
}

fn main() -> Result<(), PathError> {
    let path = Path::new("${HOME}/.zshrc");
    println!("{:?}", clean2(path)?);

    let path = Path::new("~/.zshrc");
    println!("{:?}", clean2(path)?);

    Ok(())
}
