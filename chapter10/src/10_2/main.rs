use std::{thread, time};

use nix::{fcntl, sys};

fn main() -> nix::Result<()> {
    let fd = fcntl::open(
        "Cargo.toml",
        fcntl::OFlag::O_RDONLY,
        sys::stat::Mode::empty(),
    )?;

    println!("try locking...");
    fcntl::flock(fd, fcntl::FlockArg::LockExclusive)?;
    println!("locked!");

    thread::sleep(time::Duration::from_secs(10));

    fcntl::flock(fd, fcntl::FlockArg::Unlock)?;
    println!("unlock");

    Ok(())
}
