//! ch16/getname.c
#![warn(clippy::nursery, clippy::pedantic)]
use beginning_linux_programming::inet_ntoa;

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
    let mut hostname_buf = [0_u8; 256];
    libc::gethostname(hostname_buf.as_mut_ptr().cast(), 256);
    libc::printf(
        "hostname = %s\n\0".as_ptr().cast(),
        hostname_buf.as_ptr().cast::<libc::c_char>(),
    );

    let hostinfo = *gethostbyname(hostname_buf.as_mut_ptr().cast());
    libc::printf("h_name = %s\n\0".as_ptr().cast(), hostinfo.h_name);
    // h_aliases field is list of aliases (nicknames)
    let mut alias = hostinfo.h_addr_list; // list of address (network order)
    while !alias.is_null() {
        libc::printf(
            "alias = %s\n\0".as_ptr().cast(),
            inet_ntoa(*(*alias as *mut libc::in_addr)),
        );
        alias = alias.add(1);
    }
}
