//! ch16/server5.c
use crate::inet_ntoa;
use libc::{sockaddr_in, socklen_t};
#[test]
fn main() {
    unsafe {
        server();
    }
}

#[test]
fn run_client() {
    unsafe {
        super::socket_2_tcp_echo::client();
    }
}

/// netcat localhost 8080 --tcp -vv
unsafe fn server() {
    // 1. socket
    let server_socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);

    // 2. bind
    let server_addr = super::server_sockaddr_in();
    libc::bind(
        server_socket_fd,
        (&server_addr as *const sockaddr_in).cast(),
        std::mem::size_of_val(&server_addr) as socklen_t,
    );

    // 3. listen, create a queue to store pending requests
    libc::listen(server_socket_fd, 5);

    let mut read_fds: libc::fd_set = std::mem::zeroed();
    libc::FD_ZERO(&mut read_fds);
    libc::FD_SET(server_socket_fd, &mut read_fds);
    loop {
        let mut testfds = read_fds;
        let select_res = libc::select(
            libc::FD_SETSIZE as i32,
            &mut testfds,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if select_res == -1 {
            panic!("{}", std::io::Error::last_os_error());
        }
        for fd in server_socket_fd..(libc::FD_SETSIZE as i32) {
            if !libc::FD_ISSET(fd, &mut testfds) {
                continue;
            }
            // 4. accept, return peer/client address, peer address family is same type as server bind+listen SocketAddr
            if fd == server_socket_fd {
                let mut client_addr: sockaddr_in = std::mem::zeroed();
                let mut peer_addr_len = std::mem::size_of_val(&client_addr) as socklen_t;
                let client_socket_fd = libc::accept(
                    server_socket_fd,
                    (&mut client_addr as *mut sockaddr_in).cast(),
                    &mut peer_addr_len,
                );
                libc::printf(
                    "client_addr=%s:%d, client_socket_fd=%d\n\0".as_ptr().cast(),
                    inet_ntoa(client_addr.sin_addr),
                    u32::from(client_addr.sin_port),
                    client_socket_fd,
                );
                // add new client to read_fds, and in next `loop {`(not for loop) we can read client request
                libc::FD_SET(client_socket_fd, &mut read_fds);
                break;
            }

            let mut nread: usize = 0;
            libc::ioctl(fd, libc::FIONREAD, &mut nread);
            if nread == 0 {
                println!("receive close from client_socket_fd={}", fd);
                libc::close(fd);
                libc::FD_CLR(fd, &mut read_fds);
                break;
            }
            let mut buf = [0_u8; 256];
            libc::read(fd, buf.as_mut_ptr().cast(), nread);
            println!(
                "request = {:?}\nresponse = {:?}",
                &buf[..nread],
                &buf[..nread]
            );
            libc::write(fd, (&buf as *const u8).cast(), nread);
            break;
        }
    }
}
