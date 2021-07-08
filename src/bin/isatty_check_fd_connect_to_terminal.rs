#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        _main();
    }
}

unsafe fn _main() {
    if libc::isatty(1) == 1 {
        println!("stdout is redirect to terminal");
    }
}
