use super::{server_default_sockaddr_in, MyTcpServer};
use crate::syscall;

#[test]
fn main() {
    unsafe {
        main_();
    }
}

struct Epoll {
    fd: i32,
}

impl Epoll {
    const MAX_EVENTS: usize = 100;
    fn add(&self, fd: i32) {
        let mut event = libc::epoll_event {
            events: libc::EPOLLIN as u32,
            u64: fd as u64,
        };
        syscall!(epoll_ctl(self.fd, libc::EPOLL_CTL_ADD, fd, &mut event));
    }
}

unsafe fn main_() {
    let mut server = MyTcpServer::new(true);
    server.bind_listen();
    let mut tcp_pipes = [-1; 2];
    syscall!(pipe(tcp_pipes.as_mut_ptr()));
    let addr = server_default_sockaddr_in();
    let udp_fd = syscall!(socket(
        libc::AF_INET,
        libc::SOCK_DGRAM | libc::SOCK_NONBLOCK,
        0
    ));
    syscall!(bind(
        udp_fd,
        (&addr as *const libc::sockaddr_in).cast(),
        crate::SOCKADDR_IN_LEN,
    ));
    let mut udp_pipes = [-1; 2];
    syscall!(pipe(udp_pipes.as_mut_ptr()));

    let epoll_fd = syscall!(epoll_create1(0));
    let epoll = Epoll { fd: epoll_fd };
    epoll.add(server.server_sockfd);
    epoll.add(udp_fd);
    let mut events = [std::mem::zeroed::<libc::epoll_event>(); Epoll::MAX_EVENTS];
    loop {
        let events_len = syscall!(epoll_wait(
            epoll.fd,
            events.as_mut_ptr(),
            Epoll::MAX_EVENTS as i32,
            -1
        ));
        for i in 0..events_len {
            let i = i as usize;
            if events[i].events as i32 & libc::EPOLLIN != libc::EPOLLIN {
                continue;
            }
            let fd = events[i].u64 as i32;
            if fd == server.server_sockfd {
                let conn = server.accept();
                epoll.add(conn.client_sockfd);
                std::mem::forget(conn);
            } else if fd == udp_fd {
                let mut client_addr: libc::sockaddr_in = std::mem::zeroed();
                // https://stackoverflow.com/questions/23472533/recvfrom-not-filling-the-from-ip-address-even-for-udp-messages-in-first-call
                let mut client_addr_len = crate::SOCKADDR_IN_LEN;
                let mut buf = [0_u8; 256];
                let n_read = libc::recvfrom(
                    fd,
                    buf.as_mut_ptr().cast(),
                    buf.len(),
                    0,
                    (&mut client_addr as *mut libc::sockaddr_in).cast::<libc::sockaddr>(),
                    &mut client_addr_len,
                );
                let n_write = libc::sendto(
                    fd,
                    (&buf as *const u8).cast(),
                    n_read as usize,
                    0,
                    (&client_addr as *const libc::sockaddr_in).cast(),
                    client_addr_len,
                );
                assert_eq!(n_read, n_write);
            } else {
                let mut buf = [0_u8; 256];
                loop {
                    let n_read = libc::recv(fd, buf.as_mut_ptr().cast(), buf.len(), 0);
                    if n_read == -1 {
                        let errno = *libc::__errno_location();
                        if errno == libc::EAGAIN || errno == libc::EWOULDBLOCK {
                            break;
                        }
                        unreachable!();
                    } else if n_read == 0 {
                        libc::close(fd);
                        break;
                    } else {
                        libc::send(fd, buf.as_ptr().cast(), buf.len(), 0);
                    }
                }
            }
        }
    }
}
