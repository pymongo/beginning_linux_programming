use super::MyTcpServer;
use crate::syscall;

#[test]
fn server() {
    let mut server = MyTcpServer::new(false);
    server.bind_listen();
    let conn = server.accept();

    let mut pipes = [-1; 2];
    syscall!(pipe(pipes.as_mut_ptr()));
    loop {
        // TCP recv buf -> write_pipe // without user space
        let n_read = syscall!(splice(
            conn.client_sockfd,
            std::ptr::null_mut(),
            pipes[1],
            std::ptr::null_mut(),
            libc::PIPE_BUF,
            libc::SPLICE_F_MORE | libc::SPLICE_F_MOVE
        ));
        if n_read == 0 {
            break;
        }
        // read_pipe -> TCP send buf
        syscall!(splice(
            pipes[0],
            std::ptr::null_mut(),
            conn.client_sockfd,
            std::ptr::null_mut(),
            libc::PIPE_BUF,
            libc::SPLICE_F_MORE | libc::SPLICE_F_MOVE
        ));
    }
}
