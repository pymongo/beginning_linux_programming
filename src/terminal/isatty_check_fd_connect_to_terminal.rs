#[test]
fn main() {
    unsafe {
        if libc::isatty(1) == 1 {
            println!("stdout is redirect to terminal");
        }
    }
}
