#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        #[allow(clippy::cast_ptr_alignment)]
        libc::execlp(
            // command file
            "ps\0".as_ptr().cast(),
            // arg0
            "ps\0".as_ptr().cast(),
            // arg1
            "ax\0".as_ptr().cast::<*mut libc::c_char>(),
            // NUL terminator
            0,
        );
    }
}
