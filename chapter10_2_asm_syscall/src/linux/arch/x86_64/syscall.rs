pub(crate) fn open(path: *const u8, oflag: i32) -> i32 {
    syscall2(5, path, oflag)
}

#[inline(always)]
fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;

    unsafe {
        llvm_asm!(
            "syscall " : "={rax}"(ret)
            : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2)
            : "rcx", "r11", "memory"
            : "volatile"
        );
    }

    ret
}
