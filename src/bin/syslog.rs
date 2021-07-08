#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    print_all_syslog_level();
    unsafe {
        _main();
    }
}

fn print_all_syslog_level() {
    dbg!(libc::LOG_EMERG);
    dbg!(libc::LOG_ALERT);
    dbg!(libc::LOG_CRIT);
    dbg!(libc::LOG_ERR);
    dbg!(libc::LOG_WARNING);
    dbg!(libc::LOG_NOTICE);
    dbg!(libc::LOG_INFO);
    dbg!(libc::LOG_DEBUG);
}

unsafe fn _main() {
    // %m is not a printf format, is a syslog addition
    libc::syslog(
        libc::LOG_ALERT | libc::LOG_USER,
        "hello %d, the errmsg of errno=%m\0".as_ptr().cast(),
        1,
    );
}
