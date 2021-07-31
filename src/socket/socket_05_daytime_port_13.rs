#[test]
fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    // sudo systemctl start telnet.socket
    let servinfo = *libc::getservbyname("telnet\0".as_ptr().cast(), "tcp\0".as_ptr().cast());
    let port = crate::htons(servinfo.s_port as u16);
    dbg!(port);

    let socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, libc::IPPROTO_TCP);
    let server_addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: port,
        sin_addr: libc::in_addr {
            s_addr: u32::from_be_bytes([127, 0, 0, 1]),
        },
        ..std::mem::zeroed()
    };
    crate::syscall!(connect(
        socket_fd,
        (&server_addr as *const libc::sockaddr_in).cast(),
        crate::SOCKADDR_IN_LEN,
    ));

    let mut read_buf = [0_u8; 128];
    let n_read = libc::read(socket_fd, read_buf.as_mut_ptr().cast(), read_buf.len());
    dbg!(n_read);
    libc::printf(
        "%s\0".as_ptr().cast(),
        read_buf.as_ptr().cast::<libc::c_char>(),
    );

    libc::close(socket_fd);
}
