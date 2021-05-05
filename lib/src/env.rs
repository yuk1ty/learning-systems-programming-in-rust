use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

/// Returns the path of a temporary file.
///
/// File is never created by calling this function.
///
/// # Examples
///
/// ```
/// let file = temp_file();
/// println!("Temporary file: {}", file.display());
/// ```
pub fn temp_file() -> PathBuf {
    let mut tempdir = std::env::temp_dir();

    let tempfile = {
        let now = SystemTime::now();
        let unixtime = now
            .duration_since(UNIX_EPOCH)
            .expect("system clock maybe corrupt");
        format!("{}-{:09}", unixtime.as_secs(), unixtime.subsec_nanos())
    };

    tempdir.push(tempfile);
    tempdir
}
