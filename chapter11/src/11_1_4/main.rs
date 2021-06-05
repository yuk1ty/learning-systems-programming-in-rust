use nix::unistd::getgroups;
use nix::unistd::{getgid, getuid};

fn main() {
    println!("ユーザーID: {}", getuid());
    println!("グループID: {}", getgid());

    // nix クレートにおける getgroups は Apple のプラットフォームでは実行できない。
    let groups = getgroups().unwrap();
    println!("サブグループID: {:?}", groups);
}
