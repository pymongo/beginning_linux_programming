//! ch16/client2.c, ch16/server2.c
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::doc_markdown
)]
use beginning_linux_programming::inet_ntoa;
use libc::{sockaddr_in, socklen_t};

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

const SERVER_PORT: u16 = 8080;

unsafe fn server() {
    // 1. socket
    let server_socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
    assert_ne!(server_socket_fd, -1);
    beginning_linux_programming::print_filename_from_fd(server_socket_fd);
    // set server_socket_fd to non-blocking IO
    // let flags = libc::fcntl(server_socket_fd, libc::F_GETFL, 0);
    // libc::fcntl(server_socket_fd, libc::F_SETFL, libc::O_NONBLOCK | flags);

    // 2. bind
    let server_addr = sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: SERVER_PORT,
        sin_addr: libc::in_addr { s_addr: 0 },
        // Pad to size of `struct sockaddr`
        sin_zero: [0; 8],
    };
    // The length and format of the address **depend on the address family**.
    // A particular address structure pointer will need to be **cast** to the **generic address** type (struct sockaddr *)
    libc::bind(
        server_socket_fd,
        (&server_addr as *const sockaddr_in).cast(),
        std::mem::size_of_val(&server_addr) as socklen_t,
    );

    // 3. listen, create a queue to store pending requests
    libc::listen(server_socket_fd, 1);

    // 4. accept, return peer/client address, peer address family is same type as server bind+listen SocketAddr
    // if not pending:
    //   blocking IO: requests on socket listen queue, thread would block/suspend
    //   non-blocking IO: return EWOULDBLOCK 11 Resource temporarily unavailable
    //     errno 11 is same to non-blocking read() if no data
    // client connect() syscall has a timeout
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
    beginning_linux_programming::print_filename_from_fd(socket_fd);

    // 2. connect
    let server_addr = sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: SERVER_PORT,
        sin_addr: libc::in_addr { s_addr: 0 },
        // Pad to size of `struct sockaddr`
        sin_zero: [0; 8],
    };
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
