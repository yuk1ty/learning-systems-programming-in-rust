use once_cell::sync::Lazy;
use regex::Regex;
use std::path::Path;
use std::path::PathBuf;
use std::{env, ffi::OsStr};

#[derive(Debug)]
pub enum PathError {
    IoError(std::io::Error),
    StripPrefixError(std::path::StripPrefixError),
    RegexError(regex::Error),
    VarError(std::env::VarError),
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

impl From<regex::Error> for PathError {
    fn from(error: regex::Error) -> Self {
        PathError::RegexError(error)
    }
}

impl From<std::env::VarError> for PathError {
    fn from(error: std::env::VarError) -> Self {
        PathError::VarError(error)
    }
}

static RE_VAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\{(.+)\}").unwrap());

static RE_VAR_WITHOUT_BRACES: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$(.+)").unwrap());

static TILDE: Lazy<&OsStr> = Lazy::new(|| OsStr::new("~"));

pub trait ExtendedPath {
    fn clean(&self) -> Result<PathBuf, PathError>;
    fn expand_env(&self) -> Result<PathBuf, PathError>;
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

    fn expand_env(&self) -> Result<PathBuf, PathError> {
        let mut path_buf = PathBuf::new();
        for path in self.iter() {
            let path_str = path.to_string_lossy();
            let caps_opt = RE_VAR
                .captures(&path_str)
                .or_else(|| RE_VAR_WITHOUT_BRACES.captures(&path_str));
            if let Some(caps) = caps_opt {
                let expanded_var = env::var(&caps[1])?;
                path_buf.push(expanded_var);
                continue;
            }
            if *TILDE == path {
                let home_dir = env::var("HOME")?;
                path_buf.push(home_dir);
                continue;
            }

            path_buf.push(path);
        }
        Ok(path_buf)
    }
}
