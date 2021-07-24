#![allow(clippy::doc_markdown)]
use beginning_linux_programming::syscall;

// [cargo target auto-discovery](https://doc.rust-lang.org/cargo/guide/project-layout.html)
/**
`chapter03/copy_stream.c`
哪怕用stdio.h的fgetc/fputc 一个个byte那么写，由于stdio.h内置了buffer，会比以下代码快得的
内置的buffer会例如放满1024个byte才调用一次libc::write，极大减少了系统调用次数
*/
fn main() {
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
    let read_fd = syscall!(open(read_path.as_ptr().cast(), libc::O_RDONLY));
    // std::fs::OpenOptions::create(true), std::fs::OpenOptions::truncate(true)
    // need S_IWUSR permission at first time for second time write, otherwise get errno PermissionDenied
    let write_fd = syscall!(open(
        write_path.as_ptr().cast(),
        libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
        libc::S_IRUSR | libc::S_IWUSR,
    ));

    // char buf; while(read(in,&buf,1)==1) write(out,&buf,1);
    let mut buf = 0_u8;
    loop {
        let read_len = syscall!(read(read_fd, (&mut buf as *mut u8).cast(), 1));
        if read_len == 0 {
            break;
        }
        let write_len = syscall!(write(write_fd, (&mut buf as *mut u8).cast(), 1));
        assert_eq!(write_len, 1);
    }
}
