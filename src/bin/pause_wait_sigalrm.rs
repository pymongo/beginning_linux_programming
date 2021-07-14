#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        main_();
    }
}

static mut ALARM_FIRED: bool = false;

fn sigalrm_handler(_sig: i32) {
    unsafe {
        ALARM_FIRED = true;
    }
}

unsafe fn main_() {
    let parent_pid = libc::getpid();
    dbg!(parent_pid);
    match libc::fork() {
        -1 => {
            libc::perror("fork\0".as_ptr().cast());
            libc::exit(libc::EXIT_FAILURE);
        }
        0 => {
            // child
            libc::sleep(2);
            let get_parent_pid = libc::getppid();
            dbg!(get_parent_pid);
            libc::kill(get_parent_pid, libc::SIGALRM);
            // child exit
            libc::exit(libc::EXIT_SUCCESS);
        }
        child_pid => {
            dbg!(child_pid);
        }
    }

    // only parent process/thread capture alarm signal
    libc::signal(libc::SIGALRM, sigalrm_handler as libc::sighandler_t);
    // suspend parent process/thread to wait a signal
    libc::pause();
    if ALARM_FIRED {
        println!("parent receive SIGALRM");
    }
}
