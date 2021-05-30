use std::path::Path;

fn main() {
    let path = Path::new("/folder1/folder2/example.txt");

    // Path/PathBufでは末尾と親をそれぞれ取得することができる
    let dir = path.parent().and_then(|d| d.to_str()).unwrap_or("");
    let name = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    println!("Dir: {}, Name: {}", dir, name);

    // componentsでパスを分割できる。
    let components: Vec<_> = path.components().map(|c| c.as_os_str()).collect();
    println!("{:?}", components);

    // file_nameはパスの最後の要素を返す
    println!("{}", name);

    // parentはパスのディレクトリ部分を返す
    println!("{}", dir);

    // extensionはフアイルの拡張子を返す
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    println!("{}", ext);
}
