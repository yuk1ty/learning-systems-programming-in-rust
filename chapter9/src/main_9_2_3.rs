use std::fs;

fn main() -> std::io::Result<()> {
    // フォルダを1階層だけ作成
    fs::create_dir("setting")?;

    // 深いフォルダを1回で作成
    fs::create_dir_all("setting/myapp/networksettings")?;
    Ok(())
}
