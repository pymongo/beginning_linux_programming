fn main() {
    let mut msg = "parent process";
    let mut n_times = 1;
    let mut child_process_pid = -1;
    let mut exit_code = libc::EXIT_SUCCESS;
    println!("before fork");

    match unsafe { libc::fork() } {
        -1 => unsafe {
            libc::perror("fork\0".as_ptr().cast());
            libc::exit(libc::EXIT_FAILURE);
        },
        0 => {
            msg = "child process";
            n_times = 3;
            exit_code = 37;
        }
        fork_ret_child_process_pid => {
            child_process_pid = fork_ret_child_process_pid;
        }
    }

    // parent process is exit before child process finished
    for _ in 0..n_times {
        println!("{}", msg);
        unsafe {
            libc::sleep(1);
        }
    }

    // parent
    if child_process_pid != -1 {
        let mut wait_ret_child_process_status = 0;
        let wait_ret_child_process_pid = unsafe { libc::wait(&mut wait_ret_child_process_status) };
        assert_eq!(child_process_pid, wait_ret_child_process_pid);
        if libc::WIFEXITED(wait_ret_child_process_status) {
            println!(
                "child exit code = {}",
                libc::WEXITSTATUS(wait_ret_child_process_status)
            );
        } else {
            println!("child process exit abnormally");
        }
    }

    unsafe {
        libc::exit(exit_code);
    }
}
