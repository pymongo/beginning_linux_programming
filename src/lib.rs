//! Beginning Linux Programming 4th edition exercises
#![feature(once_cell)]
#![warn(clippy::nursery, clippy::pedantic)]
//#![warn(clippy::restriction)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::doc_markdown,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::non_ascii_literal
)]
use libc::{c_char, c_int, in_addr};
// #[macro_use]
pub mod macros;
#[cfg(test)]
mod pipe;
#[cfg(test)]
mod process_and_signal;
#[cfg(test)]
mod pthread;
#[cfg(test)]
mod socket;
#[cfg(test)]
mod terminal;

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
    pub fn gethostbyname(name: *const c_char) -> *mut libc::hostent;
    /// htons: H(host byte order) TO N(network byte order) S(short)
    /// 为了考虑不同操作系统和处理器的大端序小端序可能不同，所以都转成统一的默认的 network byte order
    /// 根据 man7.org https://man7.org/linux/man-pages/man3/ntohs.3.html
    /// network byte order == MSB == bigger-endian
    pub fn htonl(hostlong: u32) -> u32;
    pub fn htons(hostshort: u16) -> u16;
}

pub const SOCKADDR_IN_LEN: libc::socklen_t =
    std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;

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
