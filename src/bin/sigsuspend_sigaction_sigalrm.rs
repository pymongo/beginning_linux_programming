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
    //libc::signal(libc::SIGALRM, sigalrm_handler as libc::sighandler_t);

    let act = libc::sigaction {
        // pinter to handler function when sig is received
        // or set to SIG_DFL(default) or SIG_IGN(ignore)
        sa_sigaction: sigalrm_handler as libc::sighandler_t,
        // 这是信号数组的感觉，可以同时监听多个信号?
        sa_mask: std::mem::zeroed(),
        sa_flags: 0,
        sa_restorer: None,
    };
    libc::sigaction(libc::SIGALRM, &act, std::ptr::null_mut());

    // suspend parent process/thread to wait a signal
    // libc::pause();
    let mut sig_mask = std::mem::zeroed();
    // block all signal
    libc::sigfillset(&mut sig_mask); // libc::sigemptyset(&mut sig_mask);
                                     // only allow SIGALRM
    libc::sigdelset(&mut sig_mask, libc::SIGALRM);
    libc::sigsuspend(&sig_mask);

    if ALARM_FIRED {
        println!("parent receive SIGALRM");
    }

    let _a = stat { id: 0 };
    let _b = stat();
    // libc::_;
}

/// Rust 很不好的设计，为了兼容 C 例如 stat 既是结构体也是函数名，Rust 也这样搞，导致静态分析变困难，ra 没法识别 `libc::sigaction` 结构体
#[allow(dead_code, non_camel_case_types)]
struct stat {
    id: i32,
}

#[allow(dead_code)]
const fn stat() -> stat {
    stat { id: 0 }
}
