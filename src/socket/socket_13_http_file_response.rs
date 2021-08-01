use super::{accept, bind_listen_default_port};
use crate::syscall;

#[test]
fn server() {
    let server_sockfd = bind_listen_default_port(false);
    let client_sockfd = accept(server_sockfd);

    let file_name = format!("{}/Cargo.toml\0", env!("CARGO_MANIFEST_DIR"));
    let mut file_stat = unsafe { std::mem::zeroed() };
    syscall!(stat(file_name.as_ptr().cast(), &mut file_stat));
    let file_size = file_stat.st_size as usize;
    let mut file_buf = vec![0_u8; file_size];
    syscall!(read(
        syscall!(open(file_name.as_ptr().cast(), libc::O_RDONLY)),
        file_buf.as_mut_ptr().cast(),
        file_size
    ));
    // HTTP 最后一个 header 的换行符是 两遍 \r\n 表示终止
    let mut resp_headers = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", file_size);
    let mut resp_body = file_buf;
    let mut iovecs = [unsafe { std::mem::zeroed::<libc::iovec>() }; 2];
    iovecs[0].iov_base = resp_headers.as_mut_ptr().cast();
    iovecs[0].iov_len = resp_headers.len();
    iovecs[1].iov_base = resp_body.as_mut_ptr().cast();
    iovecs[1].iov_len = resp_body.len();
    // 当然 header 和 body 连在一起也可以，不过那样太耦合，而且我这示例主要为了演示 writev
    unsafe {
        libc::writev(client_sockfd, iovecs.as_ptr(), iovecs.len() as i32);
    }

    // 不用 shutdown 否则要等 2*MST 才能释放服务器端口资源
    syscall!(close(client_sockfd));
    syscall!(close(server_sockfd));
}
