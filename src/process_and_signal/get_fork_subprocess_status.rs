#[test]
fn main() {
    unsafe {
        main_();
    }
}

unsafe fn print_pid_status(pid: libc::pid_t) {
    let mut buf = [0_u8; 128];
    // print PID status
    libc::sprintf(
        buf.as_mut_ptr().cast(),
        "cat /proc/%d/status | head -n 7 && echo -e '\n'\0"
            .as_ptr()
            .cast(),
        pid,
    );
    libc::system(buf.as_ptr().cast());
    libc::printf("\n\0".as_ptr().cast());
    buf = [0_u8; 128];
    // print PID task
    libc::sprintf(
        buf.as_mut_ptr().cast(),
        "ls /proc/%d/task && echo -e '\n'\0".as_ptr().cast(),
        pid,
    );
    libc::system(buf.as_ptr().cast());
}

unsafe fn main_() {
    // gettid() returns the caller's thread ID (TID). In a single-threaded process, the thread ID is equal to the process ID
    dbg!(libc::gettid());
    dbg!(libc::getpid());

    if libc::fork() == 0 {
        println!("child process: ");
        dbg!(libc::gettid());
        dbg!(libc::getpid());
        let pid = libc::getpid();
        print_pid_status(pid);
        libc::exit(libc::EXIT_SUCCESS);
    }

    // parent
    println!("parent process: ");
    let tid = libc::gettid();
    print_pid_status(tid);
}
