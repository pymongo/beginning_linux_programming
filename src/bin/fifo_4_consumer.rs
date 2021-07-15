//! ch13/fifo4.c
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        main_();
    }
}

const FILE: *const libc::c_char = "/tmp/my_pipe\0".as_ptr().cast();

#[allow(clippy::cast_sign_loss)]
unsafe fn main_() {
    if libc::access(FILE, libc::F_OK) == -1 {
        // mkfifo
        libc::mkfifo(FILE, libc::S_IRUSR | libc::S_IWUSR);
    }
    let mut buf = [0_u8; libc::PIPE_BUF];
    let fd = libc::open(FILE, libc::O_RDONLY);
    let mut total_n_read = 0;
    loop {
        let n_read = libc::read(fd, buf.as_mut_ptr().cast(), libc::PIPE_BUF) as usize;
        if n_read == 0 {
            break;
        }
        total_n_read += n_read;
    }
    dbg!(total_n_read);
}
