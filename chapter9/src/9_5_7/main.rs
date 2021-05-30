use lib::path::PathError;
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
    let image_suffix: HashSet<&'static str> = ["jpg", "jpeg", "png", "webp", "gif", "tiff", "eps"]
        .iter()
        .cloned()
        .collect();
    // ディレクトリのトラバースのためにwalkdir crateを利用する
    // ディレクトリ、拡張子が合わないものについては、フィルタして除外する
    // walkdir crateでは指定のディレクトリをスキップするような機能は存在しない
    for path in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|f| !f.file_type().is_dir())
        .filter(|f| {
            let extention_opt = f
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_lowercase());
            let extention = extention_opt.as_deref().unwrap_or("");
            image_suffix.contains(extention)
        })
    {
        println!("{}", path.into_path().display());
    }

    Ok(())
}
