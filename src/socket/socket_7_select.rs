//! ch16/server4.c
#[test]
fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let mut inputs: libc::fd_set = std::mem::zeroed();
    libc::FD_ZERO(&mut inputs);
    libc::FD_SET(libc::STDIN_FILENO, &mut inputs);

    // timeout 和 testfds 参数每次 select 之后都会变，所以每次 select 前都要重新初始化
    // 如果没有发生 timeout，timeout 好像改成还剩余多少秒
    let mut timeout = libc::timeval {
        tv_sec: 2,
        tv_usec: 500 * 1000,
    };
    let mut testfds = inputs;
    loop {
        match libc::select(
            libc::FD_SETSIZE as i32,
            &mut testfds,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut timeout,
        ) {
            0 => {
                eprintln!("select timeout!");
                std::process::exit(libc::EXIT_FAILURE);
            }
            -1 => panic!("{}", std::io::Error::last_os_error()),
            _ => {
                if libc::FD_ISSET(libc::STDIN_FILENO, &mut testfds) {
                    let mut nread: usize = 0;
                    libc::ioctl(libc::STDIN_FILENO, libc::FIONREAD, &mut nread);
                    if nread == 0 {
                        println!("EOF");
                        libc::exit(libc::EXIT_SUCCESS);
                    }
                    let mut buf = [0_u8; 256];
                    libc::read(libc::STDIN_FILENO, buf.as_mut_ptr().cast(), nread);
                    libc::printf(
                        "read from stdin: %s\n\0".as_ptr().cast(),
                        buf.as_ptr().cast::<libc::c_char>(),
                    );
                }
            }
        }
    }
}
