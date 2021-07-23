//! ch16/client3.c, ch16/server3.c
use crate::inet_ntoa;
use libc::{sockaddr_in, socklen_t};

#[test]
#[ignore = "must run both server and client"]
fn main() {
    unsafe {
        server();
    }
}

#[test]
#[ignore = "must run both server and client"]
fn run_client() {
    unsafe {
        super::socket_02_tcp_echo::client();
    }
}

unsafe fn server() {
    // 1. socket
    let server_socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);

    // 2. bind
    let server_addr = super::server_default_sockaddr_in();
    libc::bind(
        server_socket_fd,
        (&server_addr as *const sockaddr_in).cast(),
        std::mem::size_of_val(&server_addr) as socklen_t,
    );

    // 3. listen, create a queue to store pending requests
    libc::listen(server_socket_fd, 1);

    // 4. accept
    let mut client_addr: sockaddr_in = std::mem::zeroed();
    let mut peer_addr_len = std::mem::size_of_val(&client_addr) as socklen_t;
    let client_socket_fd = libc::accept(
        server_socket_fd,
        (&mut client_addr as *mut sockaddr_in).cast(),
        &mut peer_addr_len,
    );
    libc::printf(
        "client_addr=%s:%d\n\0".as_ptr().cast(),
        inet_ntoa(client_addr.sin_addr),
        libc::c_uint::from(client_addr.sin_port),
    );

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

    libc::close(server_socket_fd);
}
