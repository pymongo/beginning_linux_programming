//! ch16/server4.c
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

const SERVER_PORT: u16 = 8080;

unsafe fn server() {
    //  if the parent explicitly ignores SIGCHLD by setting its handler to SIG_IGN
    // (rather than simply ignoring the signal by default) or has the SA_NOCLDWAIT flag set,
    // all child exit status information will be discarded and no zombie processes will be left.
    libc::signal(libc::SIGCHLD, libc::SIG_IGN);

    // 1. socket
    let server_socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
    assert_ne!(server_socket_fd, -1);

    // 2. bind
    let server_addr = sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: SERVER_PORT,
        sin_addr: libc::in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    libc::bind(
        server_socket_fd,
        (&server_addr as *const sockaddr_in).cast(),
        std::mem::size_of_val(&server_addr) as socklen_t,
    );

    // 3. listen, create a queue to store pending requests
    libc::listen(server_socket_fd, 1);

    // 4. accept, return peer/client address, peer address family is same type as server bind+listen SocketAddr
    loop {
        let mut client_addr: sockaddr_in = std::mem::zeroed();
        let mut peer_addr_len = std::mem::size_of_val(&client_addr) as socklen_t;
        let client_socket_fd = libc::accept(
            server_socket_fd,
            (&mut client_addr as *mut sockaddr_in).cast(),
            &mut peer_addr_len,
        );
        if client_socket_fd == -1 {
            panic!("{}", std::io::Error::last_os_error());
        }
        libc::printf(
            "client_addr=%s:%d\n\0".as_ptr().cast(),
            inet_ntoa(client_addr.sin_addr),
            u32::from(client_addr.sin_port),
        );

        if libc::fork() == 0 {
            let mut req_buf = 0_u8;
            libc::read(
                client_socket_fd,
                (&mut req_buf as *mut u8).cast(),
                std::mem::size_of::<u8>(),
            );
            let resp = req_buf;
            println!("request = {}\nresponse = {}", req_buf, resp);
            libc::write(
                client_socket_fd,
                (&resp as *const u8).cast(),
                std::mem::size_of::<u8>(),
            );
            libc::close(client_socket_fd);
        }
    }
    // libc::close(server_socket_fd);
}
