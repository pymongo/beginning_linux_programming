use super::server_default_sockaddr_in;
use crate::syscall;
use libc::{POLLHUP, POLLIN, STDIN_FILENO, STDOUT_FILENO};

#[test]
fn telnet() {
    let addr = server_default_sockaddr_in();
    let sockfd = syscall!(socket(libc::AF_INET, libc::SOCK_STREAM, 0));
    syscall!(connect(
        sockfd,
        &addr as *const _ as *const libc::sockaddr,
        crate::SOCKADDR_IN_LEN
    ));
    let mut pipes = [-1; 2];
    syscall!(pipe(&mut pipes[0]));
    let mut pollfds = [
        libc::pollfd {
            fd: libc::STDIN_FILENO,
            events: POLLIN,
            revents: 0,
        },
        libc::pollfd {
            fd: sockfd,
            events: POLLIN | POLLHUP,
            revents: 0,
        },
    ];
    loop {
        syscall!(poll(&mut pollfds[0], pollfds.len() as libc::nfds_t, -1));
        // 跟 telnet 一样，优先判断并接收 socket 来的数据
        // 但 telnet 会把当前输入未发生的字符覆盖掉
        // 而我的程序则会保留 STDIN 的输入(尽管 terminal 显示上会被 socket 新来的数据覆盖掉)
        if pollfds[1].revents & POLLIN == POLLIN {
            syscall!(splice(
                sockfd,
                std::ptr::null_mut(),
                pipes[1],
                std::ptr::null_mut(),
                libc::PIPE_BUF,
                0
            ));
            syscall!(splice(
                pipes[0],
                std::ptr::null_mut(),
                STDOUT_FILENO,
                std::ptr::null_mut(),
                libc::PIPE_BUF,
                0
            ));
        } else if pollfds[1].revents & POLLHUP == POLLHUP {
            eprintln!("server close the connection");
        }
        if pollfds[0].revents & POLLIN == POLLIN {
            syscall!(splice(
                STDIN_FILENO,
                std::ptr::null_mut(),
                pipes[1],
                std::ptr::null_mut(),
                libc::PIPE_BUF,
                0
            ));
            syscall!(splice(
                pipes[0],
                std::ptr::null_mut(),
                sockfd,
                std::ptr::null_mut(),
                libc::PIPE_BUF,
                0
            ));
        }
    }
}
