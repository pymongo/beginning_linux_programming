//! ch13/pipe5.c
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let mut fds = [0_i32; 2];
    libc::pipe(fds.as_mut_ptr());

    if libc::fork() == 0 {
        // make dup allocate to fd=0(STDIN)
        libc::close(0);
        libc::dup(fds[0]);
        libc::close(fds[0]);
        libc::close(fds[1]);
        libc::execlp(
            "od\0".as_ptr().cast(),
            "od\0".as_ptr().cast::<libc::c_char>(),
            "-c\0".as_ptr().cast::<libc::c_char>(),
            0,
        );
        unreachable!();
    } else {
        let buf = *b"ping";
        libc::close(fds[0]);
        let n_write = libc::write(fds[1], buf.as_ptr().cast(), buf.len());
        libc::close(fds[1]);
        println!("parent write {} bytes to child via pipe", n_write);
    }
}
