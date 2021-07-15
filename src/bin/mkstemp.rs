#![warn(clippy::nursery, clippy::pedantic)]

extern "C" {
    fn mktemp(template: *mut libc::c_char) -> *mut libc::c_char;
}

/// recommend use tmpfile/mkstemp rather than tmpnam/mktemp
fn main() {
    // random tmpnam' max_len = libc::L_tmpnam
    let mut tmp_filename = [0_u8; libc::L_tmpnam as usize];
    unsafe {
        libc::tmpnam(tmp_filename.as_mut_ptr().cast());
        libc::printf(
            "tmp_filename = %s\n\0".as_ptr().cast(),
            tmp_filename.as_ptr().cast::<libc::c_char>(),
        );

        let tmpfp = libc::tmpfile();
        if tmpfp.is_null() {
            libc::perror("tmpfile".as_ptr().cast());
        } else {
            println!("tmpfile ok");
        }

        // the end of the template must as least 'X'.repeat(6)
        // only the 'X' at the end would be replace
        // char filename[] = "my_temp_file_XXXXXX";
        let mut filename = *b"/tmp/connection_XXXXXX\0";
        if libc::mkstemp(filename.as_mut_ptr().cast()) == -1 {
            libc::perror("mkstemp\0".as_ptr().cast());
            panic!();
        }
        assert_eq!(libc::unlink(filename.as_ptr().cast()), 0);

        filename = *b"/tmp/connection_XXXXXX\0";
        let mktemp_ret = mktemp(filename.as_mut_ptr().cast());
        // mktemp_ret is same as filename
        libc::printf("mktemp_ret = %s\n\0".as_ptr().cast(), mktemp_ret);
        libc::printf("filename   = %s\n\0".as_ptr().cast(), mktemp_ret);
        let fd = libc::open(mktemp_ret, libc::O_CREAT, libc::S_IRUSR | libc::S_IWUSR);
        if fd == -1 {
            libc::perror("open\0".as_ptr().cast());
            panic!();
        }
        // 由于要等所有fd关闭后文件才算被删掉，所以先unlink后close也没影响
        // 虽然磁盘没这文件(link)，但inode还没被删掉
        if libc::close(fd) == -1 {
            libc::perror("close\0".as_ptr().cast());
            panic!();
        }
        if libc::unlink(mktemp_ret) == -1 {
            libc::perror("unlink\0".as_ptr().cast());
            panic!();
        }
    }
}
