#![warn(clippy::nursery, clippy::pedantic)]
use libc::c_void;

fn main() {
    unsafe {
        main_();
    }
}

type Message = [u8; 4];

// #[no_mangle]
extern "C" fn thread_2_function(arg: *mut c_void) -> *mut c_void {
    unsafe {
        // let thread_2_arg = String::from
        println!("thread_2 receive arg = {:?}", *arg.cast::<Message>());
        libc::usleep(10 * 1000);
        let mut thread_2_output = *b"pong";
        libc::pthread_exit(thread_2_output.as_mut_ptr().cast());
    }
}

/**
除了 pthread 还有几种线程库
- C++ boost scoped thread (建议操作系统和网络基础非常好的时候才接触了解下)
- Rust 1.0 以前的 **`std::thread::scoped`** 现在叫 **`crossbeam::scoped`**

只有 scoped thread 创建的线程才能安全的传递引用
`scoped::thread` 能「保证子线程活的比主线程短」
*/
unsafe fn main_() {
    let mut thread_2: libc::pthread_t = std::mem::zeroed();
    let mut thread_2_input = *b"ping";
    println!("thread_2_input       = {:?}", thread_2_input);
    let res = libc::pthread_create(
        &mut thread_2,
        std::ptr::null(),
        thread_2_function,
        // 两个线程间传递引用是不安全的，容易出现Dangling_pointer悬垂指针，一般需要生命周期(一般是'static)
        // 潜在的 dangling_pointer 例如线程2把线程1传递的引用，修改成指向线程2的栈上局部变量，线程2结束时，引用就变成dangling_pointer
        // 又例如 子线程活的比主线程久，主线程的数据被释放，子线程依然有指向主线程数据的指针
        thread_2_input.as_mut_ptr().cast(),
    );
    assert_eq!(res, 0);
    let thread_2_output: *mut *mut c_void = std::ptr::null_mut();
    // like wait() on child process
    let join_res = libc::pthread_join(thread_2, thread_2_output);
    assert_eq!(join_res, 0);
    let thread_2_output = *(thread_2_output.cast::<Message>());
    assert_eq!(thread_2_output, *b"pong");
}
