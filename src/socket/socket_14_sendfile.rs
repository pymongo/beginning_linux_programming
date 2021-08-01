use super::MyTcpServer;
use crate::syscall;

#[test]
fn server() {
    let mut server = MyTcpServer::new(false);
    server.bind_listen();
    let conn = server.accept();

    let file_name = format!("{}/Cargo.toml\0", env!("CARGO_MANIFEST_DIR"));
    let mut file_stat = unsafe { std::mem::zeroed() };
    syscall!(stat(file_name.as_ptr().cast(), &mut file_stat));
    let file_fd = syscall!(open(file_name.as_ptr().cast(), libc::O_RDONLY));

    let resp_headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
        file_stat.st_size
    );
    syscall!(send(
        conn.client_sockfd,
        resp_headers.as_ptr().cast(),
        resp_headers.len(),
        0
    ));
    syscall!(sendfile(
        conn.client_sockfd,
        file_fd,
        std::ptr::null_mut(),
        file_stat.st_size as usize
    ));
}
