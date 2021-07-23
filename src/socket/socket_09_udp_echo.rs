use crate::inet_ntoa;
use libc::{sockaddr_in, socklen_t};

#[test]
#[ignore = "must run both server and client"]
fn main() {
    unsafe {
        udp_echo_server();
    }
}

#[test]
#[ignore = "must run both server and client"]
fn run_client() {
    unsafe {
        udp_echo_client();
    }
}

/**
如果从连接的角度来看，udp应该不属于双工。

双工的含义是：二台通讯设备之间，允许有双向的资料传输。udp在两台端设备间通信，

但与tcp不同的是，udp需要两次连接。也就是说，每一次连接都是单向传输，所以说udp更偏向单工

> netcat localhost 8080 --udp -vv
*/
unsafe fn udp_echo_server() {
    // 1. socket
    //let server_socket_fd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_UDP);
    let server_socket_fd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_IP);

    // 2. bind
    let server_addr = super::server_default_sockaddr_in();
    // The length and format of the address **depend on the address family**.
    // A particular address structure pointer will need to be **cast** to the **generic address** type (struct sockaddr *)
    let bind_res = libc::bind(
        server_socket_fd,
        (&server_addr as *const sockaddr_in).cast(),
        std::mem::size_of_val(&server_addr) as socklen_t,
    );
    dbg!(bind_res);

    // 3. listen, UDP can't listen
    // 4. accept, UDP can't accept

    loop {
        let mut client_addr: sockaddr_in = std::mem::zeroed();
        // https://stackoverflow.com/questions/23472533/recvfrom-not-filling-the-from-ip-address-even-for-udp-messages-in-first-call
        let mut client_addr_len = std::mem::size_of::<sockaddr_in>() as socklen_t;
        let mut buf = [0_u8; 256];
        let n_read = libc::recvfrom(
            server_socket_fd,
            buf.as_mut_ptr().cast(),
            buf.len(),
            0,
            (&mut client_addr as *mut sockaddr_in).cast::<libc::sockaddr>(),
            &mut client_addr_len,
        );
        libc::printf(
            "client_addr=%s:%d\n\0".as_ptr().cast(),
            inet_ntoa(client_addr.sin_addr),
            libc::c_uint::from(client_addr.sin_port),
        );
        let n_write = libc::sendto(
            server_socket_fd,
            (&buf as *const u8).cast(),
            n_read as usize,
            0,
            (&client_addr as *const sockaddr_in).cast(),
            client_addr_len,
        );
        assert_eq!(n_read, n_write);
    }
    // libc::close(server_socket_fd);
}

pub unsafe fn udp_echo_client() {
    // 1. socket
    let socket_fd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
    // beginning_linux_programming::print_filename_from_fd(socket_fd);

    // 2. connect
    let mut server_addr = super::server_default_sockaddr_in();
    let connect_res = libc::connect(
        socket_fd,
        (&server_addr as *const sockaddr_in).cast(),
        std::mem::size_of_val(&server_addr) as socklen_t,
    );
    if connect_res == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }

    let mut buf = *b"asdf";
    let n_write = libc::sendto(
        socket_fd,
        (&buf as *const u8).cast(),
        buf.len(),
        0,
        (&server_addr as *const sockaddr_in).cast(),
        std::mem::size_of_val(&server_addr) as u32,
    );
    println!("send to server = {:?}", buf);
    let mut addr_len = 0;
    let n_read = libc::recvfrom(
        socket_fd,
        (&mut buf as *mut u8).cast(),
        n_write as usize,
        0,
        (&mut server_addr as *mut sockaddr_in).cast(),
        &mut addr_len,
    );
    println!("recv from server = {:?}", buf);
    assert_eq!(n_read, n_write);
    libc::close(socket_fd);
}
