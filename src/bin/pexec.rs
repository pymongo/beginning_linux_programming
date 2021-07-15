#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        libc::execlp(
            // command file
            "ps\0".as_ptr().cast(),
            // arg0
            "ps\0".as_ptr().cast(),
            // arg1
            "ax\0".as_ptr().cast::<libc::c_char>(),
            // NUL terminator
            0,
        );
    }
}
