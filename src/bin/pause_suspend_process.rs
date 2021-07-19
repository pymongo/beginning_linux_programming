
fn main() {
    unsafe {
        dbg!(libc::getpid());
        dbg!(libc::pause());
    }
}
