use std::env;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub enum PathError {
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

pub trait ExtendedPath {
    fn clean(&self) -> Result<PathBuf, PathError>;
}

impl ExtendedPath for Path {
    fn clean(&self) -> Result<PathBuf, PathError> {
        let abs_path = self.canonicalize()?;
        if self.is_absolute() {
            return Ok(abs_path);
        }
        let current_dir = env::current_dir()?;
        abs_path
            .strip_prefix(current_dir)
            .map(|path| path.to_path_buf())
            .map_err(|e| e.into())
    }
}
