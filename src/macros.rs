/// 这个宏是为了实现对系统调用错误处理代码复用: 系统调用返回 -1 表示错误发生
/// 但是现有的 Rust 类型系统并不能抽象出这样一类函数: 任意参数类型和个数，返回值是 i32 或 isize
/// 参考 tokio/mio 代码这种系统调用错误处理代码只能通过宏去复用
/// 为了学习方便遇到错误则 panic，生产环境还是返回 Err(std::io::Error) 然后通过 ? 逐层向上传递，再通过 Error::backtrace(nightly) 找到错误行号
#[macro_export]
macro_rules! not_minus_1 {
    ( $expression:expr ) => {{
        #[allow(unused_unsafe)]
        let return_val = unsafe { $expression };
        if return_val == -1 {
            panic!(
                "error on `{}`: {}",
                stringify!($expression),
                std::io::Error::last_os_error()
            );
        } else {
            return_val
        }
    }};
}

#[test]
#[should_panic]
fn test_ret_not_minus_1() {
    not_minus_1!(libc::open(
        "/tmp/no_exist\0".as_ptr().cast(),
        libc::O_RDONLY
    ));
}
