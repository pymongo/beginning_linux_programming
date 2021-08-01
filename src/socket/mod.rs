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

#[test]
fn test_server_default_sockaddr_in() {
    for _ in 0..5 {
        let addr = server_default_sockaddr_in();
        dbg!(addr.sin_port.to_be());
    }
}
