fn main() {
    // catch_unwind 类似其它语言 try..catch 的 catch
    assert!(std::panic::catch_unwind(|| {
        panic!("oh no");
    })
    .is_err());
    println!("程序从闭包代码的 panic(\"oh no\") 这行中恢复过来，并继续往后执行");
}
