fn main() {
    let mut msg = "parent process";
    let mut n_times = 1;
    println!("before fork");

    match unsafe { libc::fork() } {
        -1 => unsafe {
            libc::perror("fork\0".as_ptr().cast());
            libc::exit(libc::EXIT_FAILURE);
        },
        0 => {
            msg = "child process";
            n_times = 3;
        }
        child_process_pid => {
            dbg!(child_process_pid);
        }
    }

    // parent process is exit before child process finished
    for _ in 0..n_times {
        println!("{}", msg);
        unsafe {
            libc::sleep(1);
        }
    }
}
