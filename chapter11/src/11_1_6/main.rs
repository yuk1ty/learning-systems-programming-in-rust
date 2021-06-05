use nix::unistd::getcwd;
use std::env::current_dir;

fn main() {
    // nix クレートを使用する場合は getcwd が使える。
    println!("{}", getcwd().unwrap().to_str().unwrap());
    // あるいは、Rust の std::env にある current_dir を使ってもよい。
    println!("{}", current_dir().unwrap().to_str().unwrap());
}
