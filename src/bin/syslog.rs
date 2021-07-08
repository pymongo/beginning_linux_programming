#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    print_all_syslog_level();
    unsafe {
        main_();
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

/// syslog.h `LOG_UPTO`
const fn log_upto(priority: i32) -> i32 {
    (1 << ((priority) + 1)) - 1
}

/// journalctl -perr --output=json-pretty -f
unsafe fn main_() {
    // ident is syslog unit name
    libc::openlog(
        "syslog.rs\0".as_ptr().cast(),
        libc::LOG_PID | libc::LOG_CONS,
        libc::LOG_USER,
    );

    // %m is not a printf format, is a syslog addition
    libc::syslog(
        libc::LOG_ERR | libc::LOG_USER,
        "hello %d, the errmsg of errno=%m\0".as_ptr().cast(),
        1,
    );

    // set max log level to libc::LOG_CRIT
    libc::setlogmask(log_upto(libc::LOG_CRIT));
    // priority > libc::LOG_CRIT would be ignore
    libc::syslog(libc::LOG_ERR, "would not show\0".as_ptr().cast());
}
