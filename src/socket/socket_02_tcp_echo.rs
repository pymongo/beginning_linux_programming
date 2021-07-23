//! ch16/client2.c, ch16/server2.c
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
        client();
    }
}

unsafe fn server() {
    // 1. socket
    let server_socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
    assert_ne!(server_socket_fd, -1);
    // set server_socket_fd to non-blocking IO
    // let flags = libc::fcntl(server_socket_fd, libc::F_GETFL, 0);
    // libc::fcntl(server_socket_fd, libc::F_SETFL, libc::O_NONBLOCK | flags);

    // arg value == 1, means true
    // libc::SO_DEBUG 选项要 sudo 权限
    let res = libc::setsockopt(
        server_socket_fd,
        libc::SOL_SOCKET,
        libc::SO_KEEPALIVE,
        (&1 as *const i32).cast::<libc::c_void>(),
        4,
    );
    if res == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }

    // 2. bind
    let server_addr = super::server_default_sockaddr_in();
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

pub unsafe fn client() {
    // 1. socket
    let socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
    // beginning_linux_programming::print_filename_from_fd(socket_fd);

    // 2. connect
    let server_addr = super::server_default_sockaddr_in();
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
