#[test]

fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let mut buf = [0_u8; libc::PIPE_BUF + 1];
    buf[0] = b'p';
    let fp = libc::popen("od -c\0".as_ptr().cast(), "w\0".as_ptr().cast());
    assert!(!fp.is_null());
    libc::fwrite(
        buf.as_mut_ptr().cast(),
        std::mem::size_of::<u8>(),
        libc::PIPE_BUF,
        fp,
    );
    libc::pclose(fp);
}
