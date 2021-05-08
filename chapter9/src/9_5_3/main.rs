use std::env;
use std::process::exit;

// whichコマンド
// PATH環境変数のパス一覧を取得してきて、split_pathsで分割
// その後、各パスの下に最初の引数で指定された実行ファイルがあるかをチェック
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{} [exec file name]", args[0]);
        exit(1);
    }

    match env::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let exec_path = path.join(&args[1]);
                if exec_path.exists() {
                    println!("{}", exec_path.to_str().expect("faled to convert to str"));
                    return;
                }
            }
        }
        None => {
            println!("failed to get PATH");
            exit(1)
        }
    }

    exit(1);
}
