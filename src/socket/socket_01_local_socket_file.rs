//! ch16/client1.c, ch16/server1.c
use crate::syscall;
#[test]
#[ignore = "must run both server and client"]
fn main() {
    server();
}

#[test]
#[ignore = "must run both server and client"]
fn run_client() {
    client();
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

## C10k problem
首先 BLOCKING 的 read/write 会阻塞线程所以同时只能处理一个 client_socket
其次每个进程最多能开 1024 个 fd，如果 10 万个客户端长链接怎么办?
```text
[w@ww bash_programming]$ ulimit -a
real-time non-blocking time  (microseconds, -R) unlimited
// ...
open files                          (-n) 1024
```
*/
fn server() {
    /* 1. socket(): create a socket endpoint for communication
    fn socket(domain: i32, type: i32, protocol: i32) -> i32: create an endpoint for communication
    domain: AF_UNIX == AF_LOCK, local communication
    type: SOCKET_STREAM Provides sequenced, reliable, two-way, connection-based byte streams. An out-of-band data transmission mechanism may be supported.
    return: fd refer to that endpoint
    */
    let server_socket_fd = syscall!(socket(libc::AF_UNIX, libc::SOCK_STREAM, 0));

    // 2. bind(): bind a name(file?) to a socket(fd)
    // AF_UNIX 的 地址最好用 sockaddr_un 的结构体，跟 AF_INET 用的 sockaddr_in 区分开
    let mut server_addr: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    server_addr.sun_family = libc::AF_UNIX as u16;
    unsafe {
        libc::strcpy(
            server_addr.sun_path.as_mut_ptr(),
            SERVER_SOCKET_FILENAME.as_ptr().cast(),
        )
    };
    // bind would create new socket_file from server_addr
    // if socket_file is exist, would get errno `Address already in use`
    syscall!(unlink(SERVER_SOCKET_FILENAME.as_ptr().cast()));
    syscall!(bind(
        server_socket_fd,
        (&server_addr as *const libc::sockaddr_un).cast(),
        crate::SOCKADDR_IN_LEN,
    ));

    // 3. listen: open connection queue and wait for clients
    // backlog arg is max pending connections in queue(queue max size)
    syscall!(listen(server_socket_fd, 5));

    // 4. accept
    let mut client_addr: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    let mut peer_addr_len = crate::SOCKADDR_IN_LEN;
    dbg!(peer_addr_len);
    let client_socket_fd = syscall!(accept(
        server_socket_fd,
        (&mut client_addr as *mut libc::sockaddr_un).cast(),
        &mut peer_addr_len,
    ));
    crate::print_filename_from_fd(client_socket_fd);
    dbg!(peer_addr_len);
    let mut buf = 0_u8;
    syscall!(read(
        client_socket_fd,
        (&mut buf as *mut u8).cast(),
        std::mem::size_of::<u8>(),
    ));
    println!("request = {}\nresponse = {}", buf, buf.to_ascii_uppercase());
    buf = buf.to_ascii_uppercase();
    syscall!(write(
        client_socket_fd,
        (&buf as *const u8).cast(),
        std::mem::size_of::<u8>(),
    ));
    syscall!(close(client_socket_fd));

    syscall!(close(server_socket_fd));
}

/** client(local socket file):
1. socket(domain, type protocol) -> socket_fd
2. connect(socket_fd, server_socket_addr, size_of_sockaddr) // connect() and bind() has same arguments
*/
fn client() {
    // 1. socket(domain, type protocol)
    let socket_fd = syscall!(socket(libc::AF_UNIX, libc::SOCK_STREAM, 0));
    crate::print_filename_from_fd(socket_fd);

    // 2. connect(socket_fd, sockaddr)
    // AF_UNIX 的 地址最好用 sockaddr_un 的结构体再 cast 成 sockaddr，跟 AF_INET 用的 sockaddr_in 区分开
    // let server_addr = libc::sockaddr {
    //     sa_family: libc::AF_UNIX as u16,
    //     sa_data: std::mem::transmute(SERVER_SOCKET_FILENAME),
    // };
    let mut server_addr: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    server_addr.sun_family = libc::AF_UNIX as u16;
    unsafe {
        libc::strcpy(
            server_addr.sun_path.as_mut_ptr(),
            SERVER_SOCKET_FILENAME.as_ptr().cast(),
        )
    };
    syscall!(connect(
        socket_fd,
        (&server_addr as *const libc::sockaddr_un).cast(),
        crate::SOCKADDR_IN_LEN,
    ));

    // after connect, socket fd is enable to use
    let mut buf = b'a';
    syscall!(write(
        socket_fd,
        (&buf as *const u8).cast(),
        std::mem::size_of::<u8>(),
    ));
    println!("request = {}", buf);
    syscall!(read(
        socket_fd,
        (&mut buf as *mut u8).cast(),
        std::mem::size_of::<u8>(),
    ));
    println!("response = {}", buf);

    syscall!(close(socket_fd));
}
