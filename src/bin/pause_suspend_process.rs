#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        dbg!(libc::getpid());
        dbg!(libc::pause());
    }
}
