#![warn(clippy::nursery, clippy::pedantic)]
use std::os::raw::{c_char, c_int};

#[link(name = "curses", kind = "dylib")]
extern "C" {
    /// TODO return *mut WINDOW
    fn initscr();
    fn endwin() -> c_int;
    fn refresh() -> c_int;
    #[link_name = "move"]
    fn move_(x: c_int, y: c_int) -> c_int;
    fn printw(format: *const c_char, ...) -> c_int;
}

fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    initscr();
    move_(10, 15);
    printw("Hello World\0".as_ptr().cast());
    refresh();
    libc::sleep(2);
    endwin();
    libc::exit(libc::EXIT_SUCCESS);
}
