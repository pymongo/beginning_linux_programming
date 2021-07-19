//! ch13/pipe1.c
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
    libc::write(fds[1], buf.as_ptr().cast(), buf_len);
    libc::read(fds[0], buf.as_mut_ptr().cast(), buf_len);
    assert_eq!(buf, *b"ping");
}
