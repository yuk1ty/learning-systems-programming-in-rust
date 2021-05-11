mod linux;

use std::{thread, time};

use linux::arch::x86_64::fcntl;

fn main_linux_x86_64() -> linux::Result<()> {
    let fd = fcntl::open_readonly("Cargo.toml")?;

    println!("try locking...");
    fcntl::flock(&fd, fcntl::FlockOperation::LockExclusive)?;
    println!("locked!");

    thread::sleep(time::Duration::from_secs(10));

    fcntl::flock(&fd, fcntl::FlockOperation::Unlock)?;
    println!("unlock");

    Ok(())
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn main() {
    main_linux_x86_64()
}

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
fn main() {
    panic!("Linux ABI, x86_64 ターゲットの `syscall` アセンブリを使うプログラムです。クロスコンパイルにも対応していません。")
}
