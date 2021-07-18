#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::doc_markdown
)]

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

const SERVER_SOCKET_FILENAME: [u8; 14] = *b"server_socket\0";

/** server(local socket file):
1. socket(domain, type protocol) -> server_socket_fd
2. bind(server_socket_fd, server_socket_addr, size_of_sockaddr) // would create socket file, filename by sockaddr.sa_data
3. listen(): open connection queue for server_socket_fd and wait for clients
4. accept() -> client_socket_fd

## server echo but uppercase client request
request: single byte
response: uppercase of request byte
*/
unsafe fn server() {
    /* 1. socket()
    fn socket(domain: i32, type: i32, protocol: i32) -> i32: create an endpoint for communication
    domain: AF_UNIX == AF_LOCK, local communication
    type: SOCKET_STREAM Provides sequenced, reliable, two-way, connection-based byte streams. An out-of-band data transmission mechanism may be supported.
    return: fd refer to that endpoint
    */
    let server_socket_fd = libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0);
    assert_ne!(server_socket_fd, -1);
    dbg!(server_socket_fd);

    // 2. bind
    let mut server_addr: libc::sockaddr_un = std::mem::zeroed();
    server_addr.sun_family = libc::AF_UNIX as u16;
    libc::strcpy(
        server_addr.sun_path.as_mut_ptr(),
        SERVER_SOCKET_FILENAME.as_ptr().cast(),
    );
    // bind would create new socket_file from server_addr
    // if socket_file is exist, would get errno `Address already in use`
    libc::unlink(SERVER_SOCKET_FILENAME.as_ptr().cast());
    let bind_res = libc::bind(
        server_socket_fd,
        (&server_addr as *const libc::sockaddr_un).cast(),
        std::mem::size_of_val(&server_addr) as libc::socklen_t,
    );
    if bind_res == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }

    // 3. listen: open connection queue and wait for clients
    // backlog arg is max pending connections in queue(queue max size)
    assert_eq!(libc::listen(server_socket_fd, 5), 0);

    // 4. accept
    let mut client_addr: libc::sockaddr_un = std::mem::zeroed();
    let mut peer_addr_len = std::mem::size_of_val(&client_addr) as libc::socklen_t;
    dbg!(peer_addr_len);
    let client_socket_fd = libc::accept(
        server_socket_fd,
        (&mut client_addr as *mut libc::sockaddr_un).cast(),
        &mut peer_addr_len,
    );
    dbg!(client_socket_fd);
    print_filename_from_fd(client_socket_fd);
    dbg!(peer_addr_len);
    if client_socket_fd == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }

    let mut buf = 0_u8;
    libc::read(
        client_socket_fd,
        (&mut buf as *mut u8).cast(),
        std::mem::size_of::<u8>(),
    );
    println!("request = {}\nresponse = {}", buf, buf.to_ascii_uppercase());
    buf = buf.to_ascii_uppercase();
    libc::write(
        client_socket_fd,
        (&buf as *const u8).cast(),
        std::mem::size_of::<u8>(),
    );
    libc::close(client_socket_fd);

    libc::close(server_socket_fd);
}

/** client(local socket file):
1. socket(domain, type protocol) -> socket_fd
2. connect(socket_fd, server_socket_addr, size_of_sockaddr) // connect() and bind() has same arguments
*/
#[cfg(test)]
unsafe fn client() {
    // 1. socket(domain, type protocol)
    let socket_fd = libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0);
    assert_ne!(socket_fd, -1);
    print_filename_from_fd(socket_fd);
    dbg!(socket_fd);

    // 2. connect(socket_fd, sockaddr)
    let server_addr = libc::sockaddr {
        sa_family: libc::AF_UNIX as u16,
        sa_data: std::mem::transmute(SERVER_SOCKET_FILENAME),
    };
    // let mut server_addr: libc::sockaddr_un = std::mem::zeroed();
    // server_addr.sun_family = libc::AF_UNIX as u16;
    // libc::strcpy(server_addr.sun_path.as_mut_ptr(), SERVER_SOCKET_FILENAME.as_ptr().cast());
    let connect_res = libc::connect(
        socket_fd,
        &server_addr,
        std::mem::size_of_val(&server_addr) as libc::socklen_t,
    );
    if connect_res == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }

    // after connect, socket fd is enable to use
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

unsafe fn print_filename_from_fd(fd: i32) {
    // linux/limits.h
    const NAME_MAX: usize = 255;
    // /proc/self is symbolic link to /proc/$PID
    // /proc/self/fd/$FD 一般都是一个软链接，如果是 socket/pipe 则会长这个样子: socket:[3428314]
    let fd_path = format!("/proc/self/fd/{}\0", fd);
    let mut buf = [0_u8; NAME_MAX];
    let n_read = libc::readlink(fd_path.as_ptr().cast(), buf.as_mut_ptr().cast(), NAME_MAX);
    if n_read == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }
    let path = String::from_utf8_unchecked(buf[..n_read as usize].to_vec());
    dbg!(path);

    // let mut stat = std::mem::zeroed();
    // libc::fstat(fd, &mut stat);
    // stat.st_dev;
}
