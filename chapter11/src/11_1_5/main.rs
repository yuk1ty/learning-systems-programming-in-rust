use nix::unistd::{getegid, geteuid, getgid, getuid};

fn main() {
    println!("ユーザー ID: {}", getuid());
    println!("グループID: {}", getgid());
    println!("実効ユーザーID: {}", geteuid());
    println!("実効グループID: {}", getegid());
}
