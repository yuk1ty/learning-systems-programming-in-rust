use std::env::temp_dir;

// ディレクトリのパスとファイル名とを連結する
fn main() {
    println!(
        "Temp File Path: {}",
        temp_dir()
            .join("temp.txt")
            .to_str()
            .expect("failed to convert to str")
    );
}
