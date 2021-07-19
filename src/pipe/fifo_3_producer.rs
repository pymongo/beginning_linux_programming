//! ch13/fifo3.c
#[test]
fn main() {
    unsafe {
        main_();
    }
}

const TOTAL_WRITE_BYTES: usize = libc::PIPE_BUF * 10_usize.pow(3);

const FILE: *const libc::c_char = "/tmp/my_pipe\0".as_ptr().cast();

unsafe fn main_() {
    if libc::access(FILE, libc::F_OK) == -1 {
        // mkfifo
        libc::mkfifo(FILE, libc::S_IRUSR | libc::S_IWUSR);
    }
    let buf = [0_u8; libc::PIPE_BUF];
    let fd = libc::open(FILE, libc::O_WRONLY);
    let mut total_n_write = 0;
    while total_n_write < TOTAL_WRITE_BYTES {
        let n_write = libc::write(fd, buf.as_ptr().cast(), libc::PIPE_BUF) as usize;
        assert_eq!(n_write, libc::PIPE_BUF);
        total_n_write += n_write;
    }
}
