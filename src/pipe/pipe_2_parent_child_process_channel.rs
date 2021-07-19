//! ch13/pipe2.c
#[test]

fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let mut fds = [0_i32; 2];
    libc::pipe(fds.as_mut_ptr());
    let mut buf = *b"ping";
    let buf_len = buf.len();

    if libc::fork() == 0 {
        let n_read = libc::read(fds[0], buf.as_mut_ptr().cast(), buf_len);
        println!("child read {} bytes from parent via pipe", n_read);
    } else {
        let n_write = libc::write(fds[1], buf.as_ptr().cast(), buf_len);
        println!("parent write {} bytes to child via pipe", n_write);
    }
}
