//! ch16/server4.c
use libc::{sockaddr_in, socklen_t};
#[test]
#[ignore = "must run both server and client"]
fn main() {
    unsafe {
        server();
    }
}

#[test]
#[ignore = "must run server first"]
fn run_client() {
    unsafe {
        super::socket_02_tcp_echo::tcp_echo_client();
    }
}

unsafe fn server() {
    //  if the parent explicitly ignores SIGCHLD by setting its handler to SIG_IGN
    // (rather than simply ignoring the signal by default) or has the SA_NOCLDWAIT flag set,
    // all child exit status information will be discarded and no zombie processes will be left.
    libc::signal(libc::SIGCHLD, libc::SIG_IGN);

    // 1. socket
    // `SOCK_CLOEXEC`: close server_socket_fd on child process by fork
    let server_socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM | libc::SOCK_CLOEXEC, 0);

    // 2. bind a name/addr to a socket
    let server_addr = super::server_default_sockaddr_in();
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
        let client_socket_fd = crate::syscall!(accept(
            server_socket_fd,
            (&mut client_addr as *mut sockaddr_in).cast(),
            &mut peer_addr_len,
        ));

        if libc::fork() == 0 {
            let mut req_buf = 0_u8;
            loop {
                let n_read = libc::read(
                    client_socket_fd,
                    (&mut req_buf as *mut u8).cast(),
                    std::mem::size_of::<u8>(),
                );
                if n_read == 0 {
                    libc::shutdown(client_socket_fd, libc::SHUT_RDWR);
                    break;
                }
                // println!("request = {}\nresponse = {}", req_buf, req_buf);
                libc::write(
                    client_socket_fd,
                    (&req_buf as *const u8).cast(),
                    std::mem::size_of::<u8>(),
                );
            }
        }
    }
}
