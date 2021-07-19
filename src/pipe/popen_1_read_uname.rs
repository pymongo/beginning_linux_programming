#[test]

fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let mut buf = [0_u8; libc::PIPE_BUF + 1];
    let fp = libc::popen("uname -a\0".as_ptr().cast(), "r\0".as_ptr().cast());
    assert!(!fp.is_null());
    // similar to std::fs::read_to_string
    libc::fread(
        buf.as_mut_ptr().cast(),
        std::mem::size_of::<u8>(),
        libc::PIPE_BUF,
        fp,
    );
    libc::printf("%s\0".as_ptr().cast(), buf.as_ptr().cast::<libc::c_char>());
    libc::pclose(fp);
}
