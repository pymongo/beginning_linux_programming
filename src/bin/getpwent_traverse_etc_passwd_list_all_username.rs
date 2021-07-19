fn main() {
    unsafe {
        libc::setpwent(); // optional, make pwent cursor to start
        loop {
            // getpwnam is get passwd by name
            let entries = libc::getpwent();
            if entries.is_null() {
                break;
            }
            libc::printf("%s\n\0".as_ptr().cast(), (*entries).pw_name);
        }
        libc::endpwent(); // optional
    }
}
