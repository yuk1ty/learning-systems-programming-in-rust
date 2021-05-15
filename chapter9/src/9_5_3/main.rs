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

    let paths = env::var_os("PATH").expect("failed to get PATH");
    env::split_paths(&paths)
        .find_map(|path| {
            let exec_path = path.join(&args[1]);
            exec_path.exists().then(|| exec_path)
        })
        .map_or_else(
            || exit(1),
            |exec_path| {
                println!("{}", exec_path.to_str().expect("faled to convert to str"));
            },
        );
}
