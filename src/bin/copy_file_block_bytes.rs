fn main() {
    unsafe {
        main_();
    }
}

/// `chapter03/copy_block.c`
#[allow(clippy::cast_sign_loss)]
unsafe fn main_() {
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
    let read_fd = libc::open(read_path.as_ptr().cast(), libc::O_RDONLY);
    assert_ne!(read_fd, -1_i32);
    let write_fd = libc::open(
        write_path.as_ptr(),
        libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
        libc::S_IRUSR | libc::S_IWUSR,
    );
    if write_fd == -1 {
        panic!("{:?}", std::io::Error::last_os_error());
    }

    let mut buf = [0_u8; BLOCK_SIZE];
    loop {
        let nread = libc::read(read_fd, buf.as_mut_ptr().cast(), BLOCK_SIZE);
        if nread <= 0 {
            break;
        }
        let nwrite = libc::write(write_fd, buf.as_mut_ptr().cast(), nread as usize);
        assert_eq!(nread, nwrite);
    }
}
