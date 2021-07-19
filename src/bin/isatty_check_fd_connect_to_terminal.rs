
fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    if libc::isatty(1) == 1 {
        println!("stdout is redirect to terminal");
    }
}
