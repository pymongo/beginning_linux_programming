//! ch16/client2.c, ch16/server2.c
use crate::{inet_ntoa, ntohs, syscall};
use libc::{sockaddr_in, socklen_t};

#[test]
#[ignore = "must run both server and client"]
fn run_server() {
    unsafe {
        tcp_echo_server();
    }
}

#[test]
#[ignore = "must run both server and client"]
fn run_client() {
    unsafe {
        tcp_echo_client();
    }
}

/**
注意 netstat 默认可能用的 WiFi 网卡，所以看不到 localhost TCP echo 的数据

同理 tcpdump 必须选择 lo 网卡才能看到 client 和 server 都是 localhost 的 echo 示例

> sudo tcpdump -i lo

```text
# 建立连接三次握手
localhost.44636 > localhost.http-alt: Flags [S], seq 2306729090, win 65495, options [mss 65495,sackOK,TS val 3066516298 ecr 0,nop,wscale 7], length 0
localhost.http-alt > localhost.44636: Flags [S.], seq 4287217975, ack 2306729091, win 65483, options [mss 65495,sackOK,TS val 3066516298 ecr 3066516298,nop,wscale 7], length 0
localhost.44636 > localhost.http-alt: Flags [.], ack 1, win 512, options [nop,nop,TS val 3066516298 ecr 3066516298], length 0

# 建立连接后，client 向 server 发一个数据包，server echo 回复一个相同的数据
localhost.44636 > localhost.http-alt: Flags [P.], seq 1:2, ack 1, win 512, options [nop,nop,TS val 3066516298 ecr 3066516298], length 1: HTTP
localhost.http-alt > localhost.44636: Flags [.], ack 2, win 512, options [nop,nop,TS val 3066516298 ecr 3066516298], length 0
localhost.http-alt > localhost.44636: Flags [P.], seq 1:2, ack 2, win 512, options [nop,nop,TS val 3066516298 ecr 3066516298], length 1: HTTP
localhost.44636 > localhost.http-alt: Flags [.], ack 2, win 512, options [nop,nop,TS val 3066516298 ecr 3066516298], length 0

# 断开连接四次挥手
localhost.http-alt > localhost.44636: Flags [F.], seq 2, ack 2, win 512, options [nop,nop,TS val 3066516298 ecr 3066516298], length 0
localhost.44636 > localhost.http-alt: Flags [F.], seq 2, ack 2, win 512, options [nop,nop,TS val 3066516298 ecr 3066516298], length 0
localhost.http-alt > localhost.44636: Flags [.], ack 3, win 512, options [nop,nop,TS val 3066516298 ecr 3066516298], length 0
localhost.44636 > localhost.http-alt: Flags [.], ack 3, win 512, options [nop,nop,TS val 3066516298 ecr 3066516298], length 0
```
*/
unsafe fn tcp_echo_server() {
    // 1. socket
    let server_socket_fd = syscall!(socket(libc::AF_INET, libc::SOCK_STREAM, 0));
    // set server_socket_fd to non-blocking IO or libc::SOCK_STREAM | libc::SOCK_NONBLOCK
    // let flags = libc::fcntl(server_socket_fd, libc::F_GETFL, 0);
    // libc::fcntl(server_socket_fd, libc::F_SETFL, libc::O_NONBLOCK | flags);
    // arg value == 1, means true
    // libc::SO_DEBUG 选项要 sudo 权限
    // syscall!(setsockopt(
    //     server_socket_fd,
    //     libc::SOL_SOCKET,
    //     libc::SO_KEEPALIVE,
    //     (&1 as *const i32).cast::<libc::c_void>(),
    //     std::mem::size_of::<i32>(),
    // ));

    // 2. bind
    let server_addr = super::server_default_sockaddr_in();
    dbg!(server_addr.sin_port);
    // The length and format of the address **depend on the address family**.
    // A particular address structure pointer will need to be **cast** to the **generic address** type (struct sockaddr *)
    libc::bind(
        server_socket_fd,
        (&server_addr as *const sockaddr_in).cast(),
        crate::SOCKADDR_IN_LEN,
    );

    // 3. listen, create a queue to store pending requests
    // get server max SYC_RECV backlog; max ESTABLISHED count is backlog+1
    let tcp_max_syn_backlog = std::fs::read_to_string("/proc/sys/net/ipv4/tcp_max_syn_backlog")
        .unwrap()
        .trim_end()
        .parse::<u16>()
        .unwrap();
    syscall!(listen(
        server_socket_fd,
        libc::c_int::from(tcp_max_syn_backlog)
    ));
    let mut addr = std::mem::zeroed();
    let mut addr_len = crate::SOCKADDR_IN_LEN;
    syscall!(getsockname(server_socket_fd, &mut addr, &mut addr_len));
    println!("addr.sa_data = {:?}", addr.sa_data);
    // syscall!(printf("%s\n\0".as_ptr().cast(), addr.sa_data));
    let addr = *(&addr as *const libc::sockaddr as *const libc::sockaddr_in);
    println!("server port = {}", ntohs(addr.sin_port));

    // 4. accept, return peer/client address, peer address family is same type as server bind+listen SocketAddr
    // if not pending:
    //   blocking IO: requests on socket listen queue, thread would block/suspend
    //   non-blocking IO: return EWOULDBLOCK 11 Resource temporarily unavailable
    //     errno 11 is same to non-blocking read() if no data
    // client connect() syscall has a timeout
    let mut client_addr: sockaddr_in = std::mem::zeroed();
    let mut peer_addr_len = std::mem::size_of_val(&client_addr) as socklen_t;
    let client_socket_fd = syscall!(accept(
        server_socket_fd,
        (&mut client_addr as *mut sockaddr_in).cast(),
        &mut peer_addr_len,
    ));
    libc::printf(
        "client_addr = %s:%d\n\0".as_ptr().cast(),
        inet_ntoa(client_addr.sin_addr),
        libc::c_uint::from(ntohs(client_addr.sin_port)),
    );
    let mut addr = std::mem::zeroed();
    let mut addr_len = crate::SOCKADDR_IN_LEN;
    syscall!(getpeername(client_socket_fd, &mut addr, &mut addr_len));
    let addr = *(&addr as *const libc::sockaddr as *const libc::sockaddr_in);
    println!("client host = {:?}", addr.sin_addr.s_addr.to_le_bytes());
    println!("client port = {}", ntohs(addr.sin_port));

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
    syscall!(close(client_socket_fd));

    // syscall!(shutdown(server_socket_fd, libc::SHUT_RDWR));
}

pub unsafe fn tcp_echo_client() {
    // 1. socket
    let socket_fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
    // beginning_linux_programming::print_filename_from_fd(socket_fd);

    // 2. connect
    let server_addr = super::server_default_sockaddr_in();
    crate::syscall!(connect(
        socket_fd,
        (&server_addr as *const sockaddr_in).cast(),
        crate::SOCKADDR_IN_LEN,
    ));

    let mut buf = b'a';
    libc::write(
        socket_fd,
        (&buf as *const u8).cast(),
        std::mem::size_of::<u8>(),
    );
    println!("reqeust = {}", buf);
    syscall!(read(
        socket_fd,
        (&mut buf as *mut u8).cast(),
        std::mem::size_of::<u8>(),
    ));
    println!("response = {}", buf);

    syscall!(shutdown(socket_fd, libc::SHUT_RDWR));
}
