use glob::{glob, Pattern, PatternError};
use std::path::PathBuf;

fn main() -> Result<(), PatternError> {
    let pattern = Pattern::new("image-*.png")?;
    println!("{}", pattern.matches("image-100.png"));

    let paths = glob("./*.png")?;
    let files: Vec<PathBuf> = paths.into_iter().filter_map(Result::ok).collect();
    println!("{:?}", files);
    Ok(())
}
