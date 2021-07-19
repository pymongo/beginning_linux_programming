use libc::c_int;

fn sigint_handler(_sigint: c_int) {
    // 软中断回调代码中一般只改全局变量的数值，不要进行 IO 等耗时操作，例如打印
    println!("receive SIGINT signal");

    // 将进程的 SIGINT 信号回调设置回默认(既收到 SIGINT 后程序立即中止)
    // 所以需要第二次 Ctrl-C 才能中止进程
    unsafe { libc::signal(libc::SIGINT, libc::SIG_DFL) };
}

/// send a Ctrl-C signal example: `kill -INT 1038868`
fn main() {
    let pid = unsafe { libc::getpid() };
    println!("PID = {}", pid);
    unsafe {
        libc::signal(libc::SIGINT, sigint_handler as libc::sighandler_t);
    }
    loop {
        println!("waiting for ctrl+c signal");
        unsafe {
            libc::usleep(1000 * 1000);
        }
    }
}
