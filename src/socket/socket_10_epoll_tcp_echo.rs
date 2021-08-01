/*!
测试方法: telnet localhost 1080
telnet send close: `Ctrl + ]` and then type `quit`

find process using TCP socket port, better than sudo lsof -i:8080:

> fuser -n tcp 8080

if you want to kill it (例如 fork的跑到一半，client关掉留下残留进程一直占用端口)

> fuser -n tcp -k 8080

# TCP 压力测试，总共发 30000 条消息(发一个'a'回一个'a')

## 1000 个 client 并发
- glommio_example_echo: 4392 ms
- socket_06_fork_multi_clients: 5483 ms # fork() 和 pthread_create 的开销应该差不多
- socket_08_select_tcp_echo: 超过 60 s
- socket_10_epoll_tcp_echo: 4243 ms

## 5000 个 client 并发
- glommio_example_echo: 4797 ms
- socket_06_fork_multi_clients: 4089 ms
- socket_10_epoll_tcp_echo: 16031 ms
*/
use crate::syscall;
use std::os::unix::prelude::RawFd;

use super::MyTcpServer;

#[test]
#[ignore = "must run both server and client"]
fn run_non_blocking() {
    reactor_main_loop();
}

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
            epoll_fd: syscall!(epoll_create1(0)),
        }
    }
}

impl Drop for Epoll {
    fn drop(&mut self) {
        syscall!(close(self.epoll_fd));
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
        syscall!(epoll_ctl(
            self.epoll_fd,
            libc::EPOLL_CTL_ADD,
            fd,
            &mut Self::event(fd, event_type)
        ));
    }

    fn remove_event(&self, fd: RawFd) {
        syscall!(epoll_ctl(
            self.epoll_fd,
            libc::EPOLL_CTL_DEL,
            fd,
            std::ptr::null_mut()
        ));
    }
}

fn reactor_main_loop() {
    let mut server = MyTcpServer::new(true);
    server.bind_listen();

    let epoll = Epoll::default();
    epoll.add_event(server.server_sockfd, libc::EPOLLIN);
    // bad example: events' len is always zero, 要么固定 1024 长度，要么每次循环 events.clear() 设置成 epoll_wait 返回值的长度
    // let mut events = Vec::<libc::epoll_event>::with_capacity(libc::FD_SETSIZE);
    let mut events = [unsafe { std::mem::zeroed() }; libc::FD_SETSIZE];

    // the event loop
    loop {
        // epoll_wait's timeout arg -1 means to block indefinitely(no timeout)
        let events_len = syscall!(epoll_wait(
            epoll.epoll_fd,
            events.as_mut_ptr(),
            libc::FD_SETSIZE as i32,
            -1
        ));
        for event in events.iter().take(events_len as usize) {
            if event.u64 == server.server_sockfd as u64 {
                let conn = server.accept();
                epoll.add_event(conn.client_sockfd, libc::EPOLLIN);
                std::mem::forget(conn);
                continue;
            }

            let fd = event.u64 as RawFd;
            // let mut buf = [0_u8; libc::BUFSIZ as usize];
            let mut buf = [0_u8; 1];
            let n_read = unsafe { libc::read(fd, buf.as_mut_ptr().cast(), buf.len()) };
            if n_read == -1 {
                panic!();
            } else if n_read == 0 {
                // The remote has closed the connection
                // println!("receive close from client_socket_fd={}", fd);
                // epoll.remove_event(fd);
                epoll.remove_event(fd);
                syscall!(close(fd));
            } else {
                let n_write = syscall!(write(fd, buf.as_ptr().cast(), n_read as usize));
                assert_eq!(n_read, n_write);
                // syscall!(printf(
                //     "received:  %s\nsend back: %s\n\0".as_ptr().cast(),
                //     buf.as_ptr().cast::<libc::c_char>(),
                //     buf.as_ptr().cast::<libc::c_char>()
                // ));
            }
        }
    }
}
