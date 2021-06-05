use nix::unistd::{getpid, getppid};

fn main() {
    // nix を使用してもよい。
    println!("プロセス ID: {}", getpid());
    // あるいは、Rust の std::process::id を使用しても取得できる。
    // println!("プロセス ID: {}", std::process::id());
    println!("親プロセスID: {}", getppid());
}
