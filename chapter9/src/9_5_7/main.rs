use lib::path::PathError;
use std::ffi::OsStr;
use std::{collections::HashSet, env};
use walkdir::WalkDir;

fn main() -> Result<(), PathError> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!(
            "Find images

Usage:
    {} [path to find]",
            &args[0]
        );
        std::process::exit(0);
    }
    let root = &args[1];
    let image_suffix: HashSet<&OsStr> = ["jpg", "jpeg", "png", "webp", "gif", "tiff", "eps"]
        .iter()
        .map(|str| OsStr::new(str))
        .collect();
    // ディレクトリのトラバースのためにworkdir crateを利用する
    // ディレクトリ、拡張子が合わないものについては、フィルタして除外する
    for path in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|f| !f.file_type().is_dir())
        .filter(|f| {
            let extention = f.path().extension().unwrap_or_else(|| OsStr::new(""));
            image_suffix.contains(extention)
        })
    {
        println!("{}", path.into_path().display());
    }

    Ok(())
}
