use super::MyTcpServer;
use crate::syscall;
use libc::{POLLERR, POLLHUP, POLLIN, POLLOUT};

#[test]
fn run_chat_server() {
    unsafe {
        chat_server();
    }
}

unsafe fn chat_server() {
    const CHAT_ROOM_SIZE: usize = 1000;
    let mut server = MyTcpServer::new(false);
    server.bind_listen();
    // let conn = server.accept();

    let mut fd_limit = std::mem::zeroed();
    syscall!(getrlimit(libc::RLIMIT_NOFILE, &mut fd_limit));
    let nfds = CHAT_ROOM_SIZE.min(fd_limit.rlim_cur as usize);

    let mut pollfds = vec![
        libc::pollfd {
            fd: -1,
            events: 0,
            revents: 0
        };
        nfds
    ];
    pollfds[0].fd = server.server_sockfd;
    pollfds[0].events = POLLIN | POLLERR;
    loop {
        syscall!(poll(pollfds.as_mut_ptr(), nfds as libc::nfds_t, -1));
        for fd in 0..nfds {
            if pollfds[fd].revents & POLLIN == POLLIN {
                // accept
                if pollfds[fd].fd == server.server_sockfd {
                    let conn = server.accept();
                    let client_sockfd = conn.client_sockfd as usize;
                    if client_sockfd > nfds {
                        drop(conn);
                        continue;
                    }
                    pollfds[client_sockfd].fd = conn.client_sockfd;
                    // set to events, get from revents
                    pollfds[client_sockfd].events = libc::POLLIN | libc::POLLERR | libc::POLLHUP;
                    std::mem::forget(conn);
                    println!("({}) join chat room", client_sockfd);
                    continue;
                }

                let mut buf = [0_u8; 128];
                let n_read = syscall!(recv(pollfds[fd].fd, buf.as_mut_ptr().cast(), buf.len(), 0));
                if n_read == 0 {
                    // receive TCP FIN from client
                    syscall!(close(pollfds[fd].fd));
                    println!("({}) left chat room", pollfds[fd].fd);
                    pollfds[fd].fd = -1;
                    continue;
                }

                let mut new_chat_msg = vec![];
                new_chat_msg.extend(format!("({}): ", fd).into_bytes());
                new_chat_msg.extend_from_slice(&buf[..n_read as usize]);
                print!("{}", String::from_utf8_unchecked(new_chat_msg.clone()));
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                #[allow(clippy::needless_range_loop)]
                for notify_fd in 0..nfds {
                    if pollfds[notify_fd].fd == -1 {
                        continue;
                    }
                    if pollfds[notify_fd].fd == server.server_sockfd {
                        continue;
                    }
                    if notify_fd == fd {
                        continue;
                    }

                    syscall!(send(
                        pollfds[notify_fd].fd,
                        new_chat_msg.as_ptr().cast(),
                        new_chat_msg.len(),
                        0
                    ));
                }
            } else if pollfds[fd].revents & POLLOUT == POLLOUT {
                unreachable!();
            } else if pollfds[fd].revents & POLLERR == POLLERR {
                // use getsockopt SO_ERROR to get error info
                panic!("error occur on fd {}", fd);
            } else if pollfds[fd].revents & POLLHUP == POLLHUP {
                panic!("client_id {} close", fd);
            }
        }
    }
}

#[test]
fn test_poll_bitwise() {
    let mut events = libc::POLLIN | libc::POLLERR;
    println!("{:08b}", events);
    // 去掉 读事件 的位运算，以便优先执行写事件
    events &= !libc::POLLIN;
    println!("{:08b}", events);
    // 添加写事件，写事件执行结束后重新把读事件加回到 event 中
    events |= libc::POLLOUT;
    println!("{:08b}", events);
}
