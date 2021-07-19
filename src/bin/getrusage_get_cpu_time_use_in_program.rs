fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let mut rusage = std::mem::zeroed();
    // libc::RUSAGE_SELF: usage information about current program
    // libc::RUSAGE_CHILDREN: usage information about current program and child processes as well
    libc::getrusage(libc::RUSAGE_SELF, &mut rusage);

    println!("user space time second:");
    dbg!(rusage.ru_utime.tv_sec);
    println!("user space time microseconds:");
    dbg!(rusage.ru_utime.tv_usec);
    println!("system/kernel space time second:");
    dbg!(rusage.ru_stime.tv_sec);
    println!("system/kernel space time microseconds:");
    dbg!(rusage.ru_stime.tv_usec);
}
