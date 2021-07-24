//! `chapter03/copy_block.c`
use beginning_linux_programming::syscall;

#[allow(clippy::cast_sign_loss)]
fn main() {
    const BLOCK_SIZE: usize = 1024;
    let target_debug_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug");
    let read_path = format!(
        "{}\0",
        target_debug_path
            .join("copy_file_single_byte")
            .to_str()
            .unwrap()
    );
    let write_path = std::ffi::CString::new(
        target_debug_path
            .join("copy_file_single_byte.bak")
            .to_str()
            .unwrap()
            .as_bytes(),
    )
    .unwrap();

    let read_fd = syscall!(open(read_path.as_ptr().cast(), libc::O_RDONLY));
    let write_fd = syscall!(open(
        write_path.as_ptr(),
        libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
        libc::S_IRUSR | libc::S_IWUSR,
    ));

    let mut buf = [0_u8; BLOCK_SIZE];
    loop {
        let nread = syscall!(read(read_fd, buf.as_mut_ptr().cast(), BLOCK_SIZE));
        if nread <= 0 {
            break;
        }
        let nwrite = syscall!(write(write_fd, buf.as_mut_ptr().cast(), nread as usize));
        assert_eq!(nread, nwrite);
    }
}
