#![warn(clippy::nursery, clippy::pedantic)]
#![feature(rustc_private)]
extern crate libc;

/// [cargo target auto-discovery](https://doc.rust-lang.org/cargo/guide/project-layout.html)
fn main() {
    unsafe { _main(); }
}

/// `chapter03/copy_stream.c`
unsafe fn _main() {
    let read_fd = libc::open("Cargo.toml\0".as_ptr().cast(), libc::O_RDONLY);
    assert_ne!(read_fd, -1_i32);
    // std::fs::OpenOptions::create(true), std::fs::OpenOptions::truncate(true)
    let write_fd = libc::open(
        "target/Cargo.toml.bak\0".as_ptr().cast(),
        libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
        libc::S_IRUSR | libc::S_IWUSR,
    );
    if write_fd == -1_i32 {
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
