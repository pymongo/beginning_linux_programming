#[test]
fn main() {
    let parked_thread = std::thread::spawn(|| {
        println!("[parked_thread]: park");
        // park函数并不能永久阻塞线程，会有一个默认的 park_timeout，也可以显式指定 park_timeout
        std::thread::park();
        // 线程会从暂停的上下文处往下执行
        println!("[parked_thread]: unparked");
    });
    std::thread::sleep(std::time::Duration::from_millis(200));
    println!("[main thread]: before unpark");
    parked_thread.thread().unpark();
    parked_thread.join().unwrap();
}
