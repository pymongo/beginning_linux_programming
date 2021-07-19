//! ch12/thread8.c
use libc::c_void;

#[test]
fn main() {
    unsafe {
        main_();
    }
}

#[allow(clippy::needless_range_loop)]
unsafe fn main_() {
    const NUM_THREADS: usize = 5;
    let mut handles = [std::mem::zeroed(); NUM_THREADS];
    for i in 0..NUM_THREADS {
        // 注意循环中给子线程必须传值，如果传引用，子线程很可能收到i+1或i+2也就是下一个循环中i的值
        libc::pthread_create(
            &mut handles[i],
            std::ptr::null(),
            thread_func,
            // void* is a addr in usize
            i as *mut libc::c_void,
        );
        println!("thread {} create success", i);
    }

    for handle in handles {
        libc::pthread_join(handle, std::ptr::null_mut());
    }

    println!("main thread 1 done");
}

#[allow(clippy::cast_sign_loss)]
extern "C" fn thread_func(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let i = arg as usize;
        libc::usleep((libc::rand() % 10_000) as libc::c_uint);
        println!("thread {} exit", i);
        libc::pthread_exit(std::ptr::null_mut());
    }
}
