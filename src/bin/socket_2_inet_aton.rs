/*!
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
1 row in set (0.000 sec)
```
*/
#![warn(clippy::nursery, clippy::pedantic)]
use libc::{c_int, c_char, in_addr};

#[link(name="c")]
extern "C" {
    /// type in_addr is a field of libc::socketadd_in, which used in bind/connect
    /// function naming is similar to atoi(string to int)
    /// aton: means string to network_ip
    /// ntoa: means network_ip to string
    /// int inet_aton (const char *cp, struct in_addr *__inp)
    fn inet_aton(cp: *const c_char, inp: *mut in_addr) -> c_int;
    /// char *inet_ntoa(struct in_addr in);
    fn inet_ntoa(in_: in_addr) -> *mut c_char;
}

fn main() {
    unsafe {
        let mut in_addr = std::mem::zeroed();
        inet_aton("192.168.1.1\0".as_ptr().cast(), &mut in_addr);
        libc::printf("%s\n\0".as_ptr().cast(), inet_ntoa(in_addr));
    }
}
