//! ch16/client3.c, ch16/server3.c
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::doc_markdown
)]
use beginning_linux_programming::inet_ntoa;
use libc::{sockaddr_in, socklen_t};

/// htons: H(host byte order) TO N(network byte order) S(short)
/// 为了考虑不同操作系统和处理器的大端序小端序可能不同，所以都转成统一的默认的 network byte order
#[link(name = "c")]
extern "C" {
    fn htonl(hostlong: u32) -> u32;
    fn htons(hostshort: u16) -> u16;
}

fn main() {
    unsafe {
        server();
    }
}

#[test]
fn run_client() {
    unsafe {
        client();
    }
}

unsafe fn server_sockaddr_in() -> sockaddr_in {
    sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: htons(8080),
        // INADDR_ANY is 0.0.0.0
        sin_addr: libc::in_addr {
            s_addr: htonl(libc::INADDR_ANY),
        },
        // Pad to size of `struct sockaddr`
        sin_zero: [0; 8],
    }
}

unsafe fn server() {
    // 1. socket
    let server_socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);

    // 2. bind
    let server_addr = server_sockaddr_in();
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
        client_addr.sin_port as libc::c_uint,
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

#[cfg(test)]
unsafe fn client() {
    // 1. socket
    let socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);

    // 2. connect
    let server_addr = server_sockaddr_in();
    let connect_res = libc::connect(
        socket_fd,
        (&server_addr as *const sockaddr_in).cast(),
        std::mem::size_of_val(&server_addr) as socklen_t,
    );
    if connect_res == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }

    let mut buf = b'a';
    libc::write(
        socket_fd,
        (&buf as *const u8).cast(),
        std::mem::size_of::<u8>(),
    );
    println!("reqeust = {}", buf);
    libc::read(
        socket_fd,
        (&mut buf as *mut u8).cast(),
        std::mem::size_of::<u8>(),
    );
    println!("response = {}", buf);

    libc::close(socket_fd);
}
