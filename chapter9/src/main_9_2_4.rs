use std::io::Write;
use std::fs;


fn open(filename: &str) -> std::io::Result<fs::File> {
    let mut f = fs::File::create(filename)?;
    let text = b"\n\
		 Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et\n\
		 dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex\n\
		 ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu\n\
		 fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt\n\
		 mollit anim id est laborum.\n\
		 ";

    f.write_all(text)?;
    Ok(f)
}

fn main() -> std::io::Result<()> {
    let filename = "server.log";
    let _ = open(filename)?;
    
    ////
    // ファイルの削除

    // ファイルや空のディレクトリの削除
    fs::remove_file(filename)?;

    fs::create_dir_all("workdir/myapp/")?;
    // ディレクトリを中身ごと削除
    fs::remove_dir_all("workdir")?;

    ////
    // 特定の長さでファイルを切り取る

    // 先頭100バイトで切る
    let file = open("truncated.txt")?;
    file.set_len(100)?;

    ////
    // ファイルを移動する/リネームする

    // リネーム
    let _ = open(filename)?;
    fs::rename(filename, "renamed.txt")?;

    // 移動
    let _ = open(filename)?;
    fs::create_dir("newdir")?;
    fs::rename(filename, "newdir/file.txt")?;

    //// デバイスやドライブが異なる場合はコピーする必要がある
    /*
    let _ = open("old_name.txt")?;
    match fs::rename(filename, "/other_device/new_file.txt") {
	Ok(_) => println!("success rename"),
	Err(_) => {
	    fs::copy("old_name.txt", "/other_device/new_file.txt")?;
	    fs::remove_file("old_name.txt")?;
	}
    }
     */
    Ok(())
}
