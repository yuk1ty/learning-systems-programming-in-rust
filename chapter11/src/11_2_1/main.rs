fn main() {
    let envs = std::env::vars();
    for (key, value) in envs {
        println!("{}={}", key, value);
    }
    // os.ExpandEnv 関数は Rust には存在しない。
}
