use std::os::unix::prelude::RawFd;

/**
try sugar 也就是问号如何知道具体是哪一个问号出错呢?
默认下只会返回一行错误消息，以及 exit(1) 的错误码
> Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
开发环境下想知道 具体哪一个?报错 和 错误的传播过程(backtrace)

首先只有 anyhow::Error 里面有 backtrace 字段记录了栈调用信息
所以「必须要每个函数返回值都是 anyhow 才能记住栈调用」
*/
fn main() -> anyhow::Result<()> {
    // std::env::set_var("RUST_BACKTRACE", "1");
    foo()?;
    Ok(())
}

fn foo() -> anyhow::Result<()> {
    let _fd = bar()?;
    Ok(())
}

fn bar() -> anyhow::Result<RawFd> {
    match unsafe { libc::open("/tmp/no_exist\0".as_ptr().cast(), libc::O_RDONLY) } {
        -1 => Err(std::io::Error::last_os_error().into()),
        fd => Ok(fd),
    }
}
