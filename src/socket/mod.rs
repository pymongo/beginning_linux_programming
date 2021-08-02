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
mod socket_13_http_writev_send_file;
mod socket_14_sendfile_without_user_space_buf;
mod socket_15_splice_without_user_space_buf_echo;
mod socket_16_poll_chat_server;
mod socket_17_poll_chat_client_telnet;

use crate::{htons, syscall};
use std::os::unix::prelude::RawFd;

pub struct MyTcpServer {
    pub server_sockfd: RawFd,
    is_non_blocking: bool,
    port: u16,
}

impl Drop for MyTcpServer {
    /// 不用 shutdown 否则要等 2*MST 才能释放服务器端口资源
    fn drop(&mut self) {
        if self.server_sockfd != -1 {
            syscall!(close(self.server_sockfd));
        }
    }
}

impl MyTcpServer {
    pub const fn new(is_non_blocking: bool) -> Self {
        Self {
            server_sockfd: -1,
            is_non_blocking,
            port: DEFAULT_SERVER_PORT,
        }
    }

    #[cfg(FALSE)]
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn bind_listen(&mut self) {
        let mut addr = server_default_sockaddr_in();
        addr.sin_port = self.port;
        let type_ = if self.is_non_blocking {
            libc::SOCK_STREAM | libc::SOCK_NONBLOCK
        } else {
            libc::SOCK_STREAM
        };
        let server_sockfd = syscall!(socket(libc::AF_INET, type_, 0));
        // reuse server socket addr and event in TIME_WAIT (server close client first)
        let enable = 1_i32;
        // libc::SO_REUSEPORT 这个是允许 IPv4 和 IPv6 绑定到同一个端口，跟 SO_REUSEADDR 不一样
        syscall!(setsockopt(
            server_sockfd,
            libc::SOL_SOCKET,
            libc::SO_REUSEADDR,
            (&enable as *const i32).cast(),
            std::mem::size_of::<libc::c_int>() as libc::socklen_t
        ));

        let server_addr = server_default_sockaddr_in();
        syscall!(bind(
            server_sockfd,
            (&server_addr as *const libc::sockaddr_in).cast(),
            crate::SOCKADDR_IN_LEN,
        ));
        // https://github.com/rust-lang/rust/blob/db492ecd5ba6bd82205612cebb9034710653f0c2/library/std/src/sys_common/net.rs#L386
        // std::net::TcpListener default backlog is 128
        let tcp_max_syn_backlog = std::fs::read_to_string("/proc/sys/net/ipv4/tcp_max_syn_backlog")
            .unwrap()
            .trim_end()
            .parse::<u16>()
            .unwrap();
        syscall!(listen(
            server_sockfd,
            libc::c_int::from(tcp_max_syn_backlog)
        ));
        self.server_sockfd = server_sockfd;
    }

    pub fn accept(&self) -> MyTcpConn {
        let mut client_addr: libc::sockaddr_in = unsafe { std::mem::zeroed() };
        // client_addr == peer_addr
        let mut peer_addr_len = crate::SOCKADDR_IN_LEN;
        let client_sockfd = syscall!(accept(
            self.server_sockfd,
            (&mut client_addr as *mut libc::sockaddr_in).cast(),
            &mut peer_addr_len,
        ));
        MyTcpConn { client_sockfd }
    }
}

pub struct MyTcpConn {
    pub client_sockfd: RawFd,
}

impl Drop for MyTcpConn {
    fn drop(&mut self) {
        syscall!(close(self.client_sockfd));
    }
}

const DEFAULT_SERVER_PORT: u16 = 8080;

pub fn server_default_sockaddr_in() -> libc::sockaddr_in {
    unsafe {
        libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: htons(DEFAULT_SERVER_PORT),
            // INADDR_ANY is 0.0.0.0
            sin_addr: libc::in_addr {
                s_addr: crate::htonl(libc::INADDR_ANY),
            },
            // Pad to make size equal to `struct sockaddr`
            ..std::mem::zeroed()
        }
    }
}
