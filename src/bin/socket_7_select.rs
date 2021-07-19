//! ch16/server4.c

fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let testfds: libc::fd_set = std::mem::zeroed();

    let mut inputs: libc::fd_set = std::mem::zeroed();
    libc::FD_ZERO(&mut inputs);
    libc::FD_SET(libc::STDIN_FILENO, &mut inputs);

    // timeout 和 testfds 参数每次 select 之后都会变，所以每次 select 前都要重新初始化
    // timeout 好像改成还剩余多少秒
    let mut timeout = libc::timeval {
        tv_sec: 2,
        tv_usec: 500 * 1000,
    };
    let mut testfds = inputs;
    match libc::select(
        libc::FD_SETSIZE as i32,
        &mut testfds,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        &mut timeout,
    ) {
        0 => panic!("timeout"),
        -1 => panic!("{}", std::io::Error::last_os_error()),
        _ => {
            if libc::FD_ISSET(libc::STDIN_FILENO, &mut testfds) {
                // libc::ioctl(fd, request)
            }
        }
    }
}
