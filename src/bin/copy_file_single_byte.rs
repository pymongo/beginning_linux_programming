#![allow(clippy::doc_markdown)]

/// [cargo target auto-discovery](https://doc.rust-lang.org/cargo/guide/project-layout.html)
fn main() {
    unsafe {
        main_();
    }
}

/**
`chapter03/copy_stream.c`
哪怕用stdio.h的fgetc/fputc 一个个byte那么写，由于stdio.h内置了buffer，会比以下代码快得的
内置的buffer会例如放满1024个byte才调用一次libc::write，极大减少了系统调用次数
*/
unsafe fn main_() {
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
    let write_path = format!(
        "{}\0",
        target_debug_path
            .join("copy_file_single_byte.bak")
            .to_str()
            .unwrap()
    );
    let read_fd = libc::open(read_path.as_ptr().cast(), libc::O_RDONLY);
    assert_ne!(read_fd, -1);
    // std::fs::OpenOptions::create(true), std::fs::OpenOptions::truncate(true)
    let write_fd = libc::open(
        write_path.as_ptr().cast(),
        libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
        libc::S_IRUSR | libc::S_IWUSR,
    );
    if write_fd == -1 {
        // need S_IWUSR permission at first time for second time write, otherwise get errno PermissionDenied
        panic!("{:?}", std::io::Error::last_os_error());
    }

    // char buf; while(read(in,&buf,1)==1) write(out,&buf,1);
    let mut buf = 0_u8;
    loop {
        let read_len = libc::read(read_fd, (&mut buf as *mut u8).cast(), 1);
        if read_len == 0 {
            break;
        }
        let write_len = libc::write(write_fd, (&mut buf as *mut u8).cast(), 1);
        assert_eq!(write_len, 1);
    }
}
