//! ch16/server5.c
use crate::syscall;
use libc::{sockaddr_in, socklen_t};
#[test]
#[ignore = "must run both server and client"]
fn main() {
    tcp_echo_select_server();
}

#[test]
#[ignore = "must run both server and client"]
fn run_client() {
    unsafe {
        super::socket_02_tcp_echo::client();
    }
}

/// telnet 127.0.0.1 8080 # telnet is only TCP
/// netcat localhost 8080 --tcp -vv
fn tcp_echo_select_server() {
    // 1. socket
    let server_socket_fd = syscall!(socket(libc::AF_INET, libc::SOCK_STREAM, libc::IPPROTO_IP));

    // 2. bind
    let server_addr = super::server_default_sockaddr_in();
    syscall!(bind(
        server_socket_fd,
        (&server_addr as *const sockaddr_in).cast(),
        std::mem::size_of_val(&server_addr) as socklen_t,
    ));

    // 3. listen, create a queue to store pending requests
    let tcp_max_syn_backlog = std::fs::read_to_string("/proc/sys/net/ipv4/tcp_max_syn_backlog")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    syscall!(listen(
        server_socket_fd,
        libc::c_int::from(tcp_max_syn_backlog)
    ));

    let mut read_fds: libc::fd_set = unsafe { std::mem::zeroed() };
    unsafe {
        libc::FD_ZERO(&mut read_fds);
        libc::FD_SET(server_socket_fd, &mut read_fds);
    }
    loop {
        let mut testfds = read_fds;
        syscall!(select(
            libc::FD_SETSIZE as i32,
            &mut testfds,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ));
        for fd in server_socket_fd..(libc::FD_SETSIZE as i32) {
            if !unsafe { libc::FD_ISSET(fd, &mut testfds) } {
                continue;
            }
            // 4. accept, return peer/client address, peer address family is same type as server bind+listen SocketAddr
            if fd == server_socket_fd {
                let mut client_addr: sockaddr_in = unsafe { std::mem::zeroed() };
                let mut peer_addr_len = std::mem::size_of_val(&client_addr) as socklen_t;
                let client_socket_fd = syscall!(accept(
                    server_socket_fd,
                    (&mut client_addr as *mut sockaddr_in).cast(),
                    &mut peer_addr_len,
                ));
                unsafe {
                    // libc::printf(
                    //     "client_addr=%s:%d, client_socket_fd=%d\n\0".as_ptr().cast(),
                    //     crate::inet_ntoa(client_addr.sin_addr),
                    //     u32::from(client_addr.sin_port),
                    //     client_socket_fd,
                    // );
                    // add new client to read_fds, and in next `loop {`(not for loop) we can read client request
                    libc::FD_SET(client_socket_fd, &mut read_fds);
                }
                break;
            }

            let mut nread: usize = 0;
            syscall!(ioctl(fd, libc::FIONREAD, &mut nread));
            if nread == 0 {
                // println!("receive close from client_socket_fd={}", fd);
                unsafe {
                    libc::close(fd);
                    libc::FD_CLR(fd, &mut read_fds);
                }
                break;
            }
            let mut buf = [0_u8; 1];
            syscall!(read(fd, buf.as_mut_ptr().cast(), nread));
            // println!(
            //     "request = {:?}\nresponse = {:?}",
            //     &buf[..nread],
            //     &buf[..nread]
            // );
            syscall!(write(fd, (&buf as *const u8).cast(), nread));
            break;
        }
    }
}
