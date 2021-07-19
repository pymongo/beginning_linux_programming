//! ch16/getdate.c
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        main_();
    }
}

#[link(name = "c")]
extern "C" {
    fn htons(hostshort: u16) -> u16;
}

/**
```text
[w@ww ~]$ cat /etc/services | grep daytime
daytime            13/tcp
daytime            13/udp
```
*/
unsafe fn main_() {
    let servinfo = *libc::getservbyname("daytime\0".as_ptr().cast(), "tcp\0".as_ptr().cast());
    dbg!(servinfo.s_port);
    dbg!(htons(servinfo.s_port as u16));

    let socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
    let server_addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: htons(servinfo.s_port as u16),
        sin_addr: libc::in_addr { s_addr: 0 },
        // Pad to size of `struct sockaddr`
        sin_zero: [0; 8],
    };
    let res = libc::connect(
        socket_fd,
        (&server_addr as *const libc::sockaddr_in).cast(),
        std::mem::size_of_val(&server_addr) as libc::socklen_t,
    );
    if res == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }
    dbg!(res);

    let mut read_buf = [0_u8; 64];
    let n_read = libc::read(socket_fd, read_buf.as_mut_ptr().cast(), 64);
    dbg!(n_read);
    libc::printf(
        "%s\0".as_ptr().cast(),
        read_buf.as_ptr().cast::<libc::c_char>(),
    );

    libc::close(socket_fd);
}
