//! Beginning Linux Programming 4th edition exercises
#![warn(clippy::nursery, clippy::pedantic)]
//#![warn(clippy::restriction)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::doc_markdown,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc
)]
use libc::{c_char, c_int, in_addr};
#[cfg(test)]
mod pipe;
#[cfg(test)]
mod pthread;
#[cfg(test)]
mod socket;

/**
```text
MariaDB [test]> select inet_aton("192.168.1.1");
+--------------------------+
| inet_aton("192.168.1.1") |
+--------------------------+
|               3232235777 |
+--------------------------+
1 row in set (0.000 sec)

MariaDB [test]> select inet_ntoa(3232235777);
+-----------------------+
| inet_ntoa(3232235777) |
+-----------------------+
| 192.168.1.1           |
+-----------------------+
```
*/
#[link(name = "c")]
extern "C" {
    /// type in_addr is a field of libc::socketadd_in, which used in bind/connect
    /// function naming is similar to atoi(string to int)
    /// aton: means string to network_ip
    /// ntoa: means network_ip to string
    /// int inet_aton (const char *cp, struct in_addr *__inp)
    pub fn inet_aton(cp: *const c_char, inp: *mut in_addr) -> c_int;
    /// char *inet_ntoa(struct in_addr in);
    pub fn inet_ntoa(in_: in_addr) -> *mut c_char;
    // inet_addr 是 inet_aton 完全不考虑字符串解析错误的版本
    // fn inet_addr(cp: *const c_char) -> in_addr_t
    /// struct hostent *gethostbyname(const char *name);
    pub fn gethostbyname(name: *const libc::c_char) -> *mut libc::hostent;
    /// htons: H(host byte order) TO N(network byte order) S(short)
    /// 为了考虑不同操作系统和处理器的大端序小端序可能不同，所以都转成统一的默认的 network byte order
    pub fn htonl(hostlong: u32) -> u32;
    pub fn htons(hostshort: u16) -> u16;
}

#[test]
fn test_inet_aton() {
    unsafe {
        let mut in_addr = std::mem::zeroed();
        inet_aton("127.0.0.1\0".as_ptr().cast(), &mut in_addr);
        libc::printf("%s\n\0".as_ptr().cast(), inet_ntoa(in_addr));
        dbg!(in_addr.s_addr);
        dbg!(in_addr.s_addr.to_be_bytes());
        dbg!(in_addr.s_addr.to_le_bytes());
        dbg!((1_u32 << 24) + 127);
        assert_eq!(in_addr.s_addr, (1_u32 << 24) + 127);
    }
}

pub unsafe fn print_filename_from_fd(fd: i32) {
    // linux/limits.h
    const NAME_MAX: usize = 255;
    // /proc/self is symbolic link to /proc/$PID
    // /proc/self/fd/$FD 一般都是一个软链接，如果是 socket/pipe 则会长这个样子: socket:[3428314]
    let fd_path = format!("/proc/self/fd/{}\0", fd);
    let mut buf = [0_u8; NAME_MAX];
    let n_read = libc::readlink(fd_path.as_ptr().cast(), buf.as_mut_ptr().cast(), NAME_MAX);
    if n_read == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }
    let path = String::from_utf8_unchecked(buf[..n_read as usize].to_vec());
    dbg!(path);
}
