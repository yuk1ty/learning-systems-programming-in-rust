#[cfg(not(any(target_os = "ios", target_os = "macos")))]
use nix::unistd::getgroups;
use nix::unistd::{getgid, getuid};

fn main() {
    println!("ユーザーID: {}", getuid());
    println!("グループID: {}", getgid());

    // nix クレートにおける getgroups は Apple のプラットフォームでは実行できない。
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    get_groups();
}

#[cfg(not(any(target_os = "ios", target_os = "macos")))]
fn get_groups() {
    let groups = getgroups().unwrap();
    println!("サブグループID: {:?}", groups);
}
