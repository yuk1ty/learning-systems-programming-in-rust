use nix::sys::wait::*;
use nix::unistd::{execve, fork, ForkResult};
use std::env;
use std::ffi::CString;

// Go のプログラミングでは触れることのない世界として言及されているが、Rust では触れることができる。
// 本書の中で fork(2) と execve(3) が言及されていたので、それを使用したサンプルプログラムを掲載しておく。
fn main() {
    // fork
    match unsafe { fork() }.expect("fork failed") {
        ForkResult::Parent { child } => {
            // waitpid
            match waitpid(child, None).expect("wait_pid failed") {
                WaitStatus::Exited(pid, status) => {
                    println!("exit!: pid={:?}, status={:?}", pid, status)
                }
                WaitStatus::Signaled(pid, status, _) => {
                    println!("signal!: pid={:?}, status={:?}", pid, status)
                }
                _ => println!("abnormal exit!"),
            }
        }
        ForkResult::Child => {
            // 引数の値を取得する。
            let args: Vec<String> = env::args().collect();
            let dir = CString::new(args[1].to_string()).unwrap();
            let arg = CString::new(args[2].to_string()).unwrap();
            // env は仮で入れておく。
            let env = CString::new("ENV=prd".to_string()).unwrap();

            // execv
            execve(&dir, &[dir.clone(), arg], &[env]).expect("execution failed.");
        }
    }
}
