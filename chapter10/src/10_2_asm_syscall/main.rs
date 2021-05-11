#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn main() {
    println!("Hello, x86_64 Linux.");
}

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
fn main() {
    panic!("Linux ABI, x86_64 ターゲットの `syscall` アセンブリを使うプログラムです。クロスコンパイルにも対応していません。")
}
