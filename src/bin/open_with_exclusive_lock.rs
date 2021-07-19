
fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    const FILENAME: *const libc::c_char = "/tmp/my_cargo.lock\0".as_ptr().cast();
    let mut fd = -1;
    for _ in 0..60 {
        // open in read/write and exclusive
        fd = libc::open(FILENAME, libc::O_RDWR | libc::O_CREAT | libc::O_EXCL);
        if fd != -1 {
            break;
        }
        if *libc::__errno_location() == libc::EEXIST {
            // \033(oct) is same to \x1b(hex)
            println!("    \x1b[1;36mBlocking\x1b[0m waiting for file lock on build directory");
        }
        libc::sleep(2);
    }
    for built_crates in 0..=5 {
        let mut progress = "=".repeat(built_crates * 2);
        progress.push('>');
        println!(
            "    \x1b[1;36mBuilding\x1b[0m [{:<11}] {}/5",
            progress, built_crates
        );
        libc::sleep(1);
    }
    println!("    \x1b[1;32mFinished\x1b[0m dev [unoptimized + debuginfo] target(s) in 5.00s");
    libc::close(fd);
    libc::unlink(FILENAME);
}
