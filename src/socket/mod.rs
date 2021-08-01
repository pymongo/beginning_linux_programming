mod socket_01_local_socket_file;
mod socket_02_tcp_echo;
mod socket_03_get_host_byte_order;
mod socket_04_gethostname;
mod socket_05_daytime_port_13;
mod socket_06_fork_multi_clients;
mod socket_07_select;
mod socket_08_select_tcp_echo;
mod socket_09_udp_echo;
mod socket_10_epoll_tcp_echo;
mod socket_11_accept_async_reactor_wake_future;
mod socket_12_ping_icmp_protocol;
mod socket_13_http_file_response;

use crate::syscall;
use std::os::unix::prelude::RawFd;

const SERVER_PORT: u16 = 8080;

pub fn server_default_sockaddr_in() -> libc::sockaddr_in {
    unsafe {
        libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: crate::htons(SERVER_PORT),
            // INADDR_ANY is 0.0.0.0
            sin_addr: libc::in_addr {
                s_addr: crate::htonl(libc::INADDR_ANY),
            },
            ..std::mem::zeroed()
            // Pad to size of `struct sockaddr`
            // sin_zero: [0; 8],
        }
    }
}

/// 完成 std::net::TcpListener::bind() 的操作，并返回 server 的 socket_fd
pub fn bind_listen_default_port(non_blocking: bool) -> RawFd {
    let type_ = if non_blocking {
        libc::SOCK_STREAM | libc::SOCK_NONBLOCK
    } else {
        libc::SOCK_STREAM
    };
    let server_socket_fd = syscall!(socket(libc::AF_INET, type_, 0));
    let server_addr = server_default_sockaddr_in();
    syscall!(bind(
        server_socket_fd,
        (&server_addr as *const libc::sockaddr_in).cast(),
        crate::SOCKADDR_IN_LEN,
    ));
    // https://github.com/rust-lang/rust/blob/db492ecd5ba6bd82205612cebb9034710653f0c2/library/std/src/sys_common/net.rs#L386
    // std::net::TcpListener default backlog is 128
    syscall!(listen(server_socket_fd, 128));
    // set_nonblocking(server_socket_fd);
    server_socket_fd
}

/// input_arg: server_fd, return client_socket_fd
/// TCP accept 之后得到 client_socket_fd 就可以通过 fd 进行全双工通信了
pub fn accept(server_socket_fd: RawFd) -> RawFd {
    let mut client_addr: libc::sockaddr_in = unsafe { std::mem::zeroed() };
    // client_addr == peer_addr
    let mut peer_addr_len = crate::SOCKADDR_IN_LEN;
    let client_socket_fd = syscall!(accept(
        server_socket_fd,
        (&mut client_addr as *mut libc::sockaddr_in).cast(),
        &mut peer_addr_len,
    ));
    client_socket_fd
}
