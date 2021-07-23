#[test]
fn main() {
    unsafe {
        main_();
    }
}

/// 由于 -1 是一个 valid 的 getpriority 返回值，所以只能先把errno设置成0再调用getpriority看errno有没有变成其他错误码
/// errno should be set to zero before calling getpriority and checked that it’s still zero
unsafe fn main_() {
    *libc::__errno_location() = 0;
    // default process_priority is 0, priority range [-20, 20], -20 is the highest priority
    let systemd_process_priority = libc::getpriority(libc::PRIO_PROCESS, 1);
    if *libc::__errno_location() > 0 {
        libc::perror("getpriority\0".as_ptr().cast());
    }
    dbg!(systemd_process_priority);

    *libc::__errno_location() = 0;
    let current_process_priority = libc::getpriority(libc::PRIO_PROCESS, libc::getpid() as u32);
    if *libc::__errno_location() > 0 {
        libc::perror("getpriority\0".as_ptr().cast());
    }
    dbg!(current_process_priority);
}
