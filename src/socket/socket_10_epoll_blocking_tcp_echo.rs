use crate::not_minus_1;
use std::os::unix::prelude::RawFd;

// struct Reactor {
//     epoll: Epoll
// }
struct Epoll {
    epoll_fd: RawFd,
}

/// epoll_create1 的 flag 参数: EPOLL_CLOEXEC(O_CLOEXEC) 的作用: 进行类似 exec 系统调用时关闭所有 O_CLOEXEC 标记的 fd
/// 由于我们应用没有涉及 exec 所以不需要设置该 flag
/// mio::Registry == mio/src/sys/unix/selector/epoll.rs == all epoll syscall
impl Default for Epoll {
    fn default() -> Self {
        Self {
            epoll_fd: not_minus_1!(libc::epoll_create1(0)),
        }
    }
}

impl Drop for Epoll {
    fn drop(&mut self) {
        not_minus_1!(libc::close(self.epoll_fd));
    }
}

impl Epoll {
    /// private
    const fn event(fd: RawFd, event_type: libc::c_int) -> libc::epoll_event {
        libc::epoll_event {
            /* events field:
            - EPOLLIN/EPOLLOUT: read/write
            - EPOLLET(epoll edge-triggered):
                例如 nginx 会用边缘触发等内核缓冲区满才通知，避免频繁读写。
                我们应用就用默认的水平触发够了(有数据就通知)
            */
            events: event_type as u32,
            /* u64 field:
            u64 字段在 mio 中是 mio::Token 的抽象
            由于 epoll_ctl 已经有一个 fd 的入参，所以这个 u64 其实是给调用者放一些额外的信息，例如 mio::Token 抽象信息
            为了简单，我们应用就把 u64 字段
            */
            u64: fd as u64,
        }
    }

    fn add_event(&self, fd: RawFd, event_type: libc::c_int) {
        not_minus_1!(libc::epoll_ctl(
            self.epoll_fd,
            libc::EPOLL_CTL_ADD,
            fd,
            &mut Self::event(fd, event_type)
        ));
    }

    fn remove_event(&self, fd: RawFd) {
        not_minus_1!(libc::epoll_ctl(
            self.epoll_fd,
            libc::EPOLL_CTL_DEL,
            fd,
            std::ptr::null_mut()
        ));
    }
}

const SOCKADDR_INET_SIZE: libc::socklen_t =
    std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;

/// 完成 std::net::TcpListener::bind() 的操作，并返回 server 的 socket_fd
fn bind_listen_default_port() -> RawFd {
    let server_socket_fd = not_minus_1!(libc::socket(
        libc::AF_INET,
        libc::SOCK_STREAM,
        libc::IPPROTO_IP
    ));
    let server_addr = super::server_default_sockaddr_in();
    not_minus_1!(libc::bind(
        server_socket_fd,
        (&server_addr as *const libc::sockaddr_in).cast(),
        SOCKADDR_INET_SIZE,
    ));
    // https://github.com/rust-lang/rust/blob/db492ecd5ba6bd82205612cebb9034710653f0c2/library/std/src/sys_common/net.rs#L386
    // std::net::TcpListener default backlog is 128
    not_minus_1!(libc::listen(server_socket_fd, 128));
    // set_nonblocking(server_socket_fd);
    server_socket_fd
}

fn set_nonblocking(fd: RawFd) {
    let flags = not_minus_1!(libc::fcntl(fd, libc::F_GETFL, 0));
    not_minus_1!(libc::fcntl(fd, libc::F_SETFL, libc::O_NONBLOCK | flags));
}

/// input_arg: server_fd, return client_socket_fd
/// TCP accept 之后得到 client_socket_fd 就可以通过 fd 进行全双工通信了
fn accept(server_socket_fd: RawFd) -> RawFd {
    let mut client_addr: libc::sockaddr_in = unsafe { std::mem::zeroed() };
    // client_addr == peer_addr
    let mut peer_addr_len = std::mem::size_of_val(&client_addr) as libc::socklen_t;
    let client_socket_fd = not_minus_1!(libc::accept(
        server_socket_fd,
        (&mut client_addr as *mut libc::sockaddr_in).cast(),
        &mut peer_addr_len,
    ));
    unsafe {
        libc::printf(
            "client_addr=%s:%d, client_socket_fd=%d\n\0".as_ptr().cast(),
            crate::inet_ntoa(client_addr.sin_addr),
            u32::from(client_addr.sin_port),
            client_socket_fd,
        );
    }
    client_socket_fd
}

#[derive(Debug)]
#[repr(C)]
pub struct ev {
    pub events: u32,
    pub u64: u64,
}

#[test]
fn reactor_main_loop() {
    let server_socket_fd = bind_listen_default_port();
    let epoll = Epoll::default();
    epoll.add_event(server_socket_fd, libc::EPOLLIN);
    // bad example: events' len is always zero, 要么固定 1024 长度，要么每次循环 events.clear() 设置成 epoll_wait 返回值的长度
    // let mut events = Vec::<libc::epoll_event>::with_capacity(libc::FD_SETSIZE);
    let mut events = vec![unsafe { std::mem::zeroed() }; libc::FD_SETSIZE];

    // the event loop
    loop {
        // epoll_wait's timeout arg -1 means to block indefinitely(no timeout)
        let events_len = not_minus_1!(libc::epoll_wait(
            epoll.epoll_fd,
            events.as_mut_ptr(),
            libc::FD_SETSIZE as i32,
            -1
        ));
        dbg!(events_len);
        for event in events.iter().take(events_len as usize) {
            if event.u64 == server_socket_fd as u64 {
                let client_socket_fd = accept(server_socket_fd);
                set_nonblocking(client_socket_fd);
                epoll.add_event(client_socket_fd, libc::EPOLLIN);
                // because event is clear each loop, need to add server_socket_fd event again
                // epoll.add_event(server_socket_fd, libc::EPOLLIN);
                continue;
            }

            let fd = event.u64 as RawFd;
            let mut buf = [0_u8; libc::BUFSIZ as usize];
            let n_read = unsafe { libc::read(fd, buf.as_mut_ptr().cast(), buf.len()) };
            if n_read == -1 {
                if unsafe { *libc::__errno_location() } == libc::EAGAIN {
                    panic!();
                }
                panic!();
            } else if n_read == 0 {
                // The remote has closed the connection
                println!("receive close from client_socket_fd={}", fd);
                // epoll.remove_event(fd);
                epoll.remove_event(fd);
                not_minus_1!(libc::close(fd));
            } else {
                let n_write = not_minus_1!(libc::write(fd, buf.as_ptr().cast(), n_read as usize));
                assert_eq!(n_read, n_write);
            }
        }
    }
}
