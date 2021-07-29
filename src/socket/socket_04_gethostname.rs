//! ch16/getname.c
use crate::{htonl, inet_ntoa, NAME_MAX};

#[test]
fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let mut hostname_buf = [0_u8; NAME_MAX];
    libc::gethostname(hostname_buf.as_mut_ptr().cast(), NAME_MAX);
    libc::printf(
        "hostname = %s\n\0".as_ptr().cast(),
        hostname_buf.as_ptr().cast::<libc::c_char>(),
    );

    let hostinfo = *crate::gethostbyname(hostname_buf.as_ptr().cast());
    libc::printf("h_name = %s\n\0".as_ptr().cast(), hostinfo.h_name);
    // h_aliases field is list of aliases (nicknames)
    let mut alias = hostinfo.h_addr_list; // list of address (network order)
    while !alias.is_null() {
        let in_addr_ptr = (*alias).cast::<[u8; 4]>();
        if in_addr_ptr.is_null() {
            break;
        }
        let in_addr = libc::in_addr {
            s_addr: { htonl(u32::from_be_bytes(*in_addr_ptr)) },
        };
        libc::printf("alias = %s\n\0".as_ptr().cast(), inet_ntoa(in_addr));
        alias = alias.add(1);
    }
}
