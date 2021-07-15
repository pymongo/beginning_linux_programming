#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    // add more nul type to printf
    let mut buf = [0_u8; libc::PIPE_BUF + 1];
    let fp = libc::popen("ps ax\0".as_ptr().cast(), "r\0".as_ptr().cast());
    loop {
        let n_read = libc::fread(
            buf.as_mut_ptr().cast(),
            std::mem::size_of::<u8>(),
            libc::PIPE_BUF,
            fp,
        );
        if n_read == 0 {
            break;
        }
        // 将已读长度的后一位设为NULL，避免第二次读取后打印字符串时打印的长度不是 n_read (把n_read后面上次buffer残留数据也打印了)
        buf[n_read] = b'\0';
        libc::printf("%s\0".as_ptr().cast(), buf.as_ptr().cast::<libc::c_char>());
    }
    libc::pclose(fp);
}
