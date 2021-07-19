mod socket_1_local_socket_file;
mod socket_2_tcp_echo;
mod socket_3_htonl;
mod socket_4_gethostname;
mod socket_5_daytime_port_13;
mod socket_6_fork_multi_clients;
mod socket_7_select;
mod socket_8_select_multi_clients;

const SERVER_PORT: u16 = 8080;

pub unsafe fn server_sockaddr_in() -> libc::sockaddr_in {
    libc::sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: crate::htons(SERVER_PORT),
        // INADDR_ANY is 0.0.0.0
        sin_addr: libc::in_addr {
            s_addr: crate::htonl(libc::INADDR_ANY),
        },
        // Pad to size of `struct sockaddr`
        sin_zero: [0; 8],
    }
}
