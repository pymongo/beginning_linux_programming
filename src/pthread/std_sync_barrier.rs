/// 通过 Barrier 让乱序执行的 5 个线程强行"有序"，5 个线程全部执行完前半部分操作后，再开始后半部分操作
/// 例如学校组织同学们去春游或公司团建，必须等班上所有同学都上了大巴车之后，大巴才能发车去下一个景点，不会让任何同学(线程)掉队
#[test]
fn main() {
    const N_THREADS: usize = 5;
    let mut handles = Vec::with_capacity(N_THREADS);
    // Creates a new barrier that can block a given number of threads
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(N_THREADS));
    for i in 0..N_THREADS {
        let barrier_clone = barrier.clone();
        let handle = std::thread::Builder::new()
            .name(format!("thread_{}", i))
            .spawn(move || {
                println!("{}: Before wait", std::thread::current().name().unwrap());
                barrier_clone.wait();
                println!("{}: After wait", std::thread::current().name().unwrap());
            })
            .unwrap();
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
