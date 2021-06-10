use nix::unistd::{getpgrp, getpid, getsid};

fn main() {
    let sid = getsid(Some(getpid())).unwrap();
    println!("グループID: {}, セッションID: {}", getpgrp(), sid);
}
