use nix::libc::getppid;
// gopsutil の Rust バージョンとして rust-psutil というクレートを使用できそうだったが、
// 実装がかなり不完全そうだった。
// そこで、sysinfo というクレートを使っている。
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    // get_process に libc の pid_t を要求するので、それを取得できるように nix::libc 経由で getppid を使っている。
    let pid = unsafe { getppid() };
    if let Some(process) = System::new_all().get_process(pid) {
        println!(
            "parent pid: {}, name: {}, cmd: {:?}",
            process.pid(),
            process.name(),
            process.cmd()
        )
    }
}
