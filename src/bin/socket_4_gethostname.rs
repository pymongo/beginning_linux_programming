//! ch16/getname.c
#![warn(clippy::nursery, clippy::pedantic)]

#[link(name = "c")]
extern "C" {
    /// struct hostent *gethostbyname(const char *name);
    fn gethostbyname(name: *const libc::c_char) -> *mut libc::hostent;
}

fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let mut name_buf = [0_u8; 256];
    libc::gethostname(name_buf.as_mut_ptr().cast(), 256);
    libc::printf(
        "hostname = %s\n\0".as_ptr().cast(),
        name_buf.as_ptr().cast::<libc::c_char>(),
    );

    // libc::gethostbyname();

    // libc::servent;
    // libc::hostent;
}
