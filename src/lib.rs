//! Beginning Linux Programming 4th edition exercises
#![warn(clippy::nursery, clippy::pedantic)]
//#![warn(clippy::restriction)]
use libc::{c_char, c_int, in_addr};

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
}

#[test]
fn test_inet_aton() {
    unsafe {
        let mut in_addr = std::mem::zeroed();
        inet_aton("192.168.1.1\0".as_ptr().cast(), &mut in_addr);
        libc::printf("%s\n\0".as_ptr().cast(), inet_ntoa(in_addr));
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
