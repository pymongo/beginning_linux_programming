//! ch12/thread7.c
use libc::{c_int, c_void};

#[test]
fn main() {
    unsafe {
        main_();
    }
}

/// 由于是更早版本的 Linux 没有这个 API, 为了兼容老的 Linux 机器能编译 libc ，所以没加这个 API
#[link(name = "pthread")]
extern "C" {
    fn pthread_setcanceltype(new_cancel_type: c_int, old_cancel_type: *mut c_int) -> c_int;
}

// const PTHREAD_CANCEL_DEFERRED: c_int = 0;
const PTHREAD_CANCEL_ASYNCHRONOUS: c_int = 1;

unsafe fn main_() {
    let mut thread2_handle = std::mem::zeroed();
    // pthread_create handle can't use null ptr
    libc::pthread_create(
        &mut thread2_handle,
        std::ptr::null(),
        thread_2_function,
        std::ptr::null_mut(),
    );
    libc::usleep(100);
    libc::pthread_cancel(thread2_handle);
    // PTHREAD_CANCEL_ASYNCHRONOUS cancel immediately, doesn't need to defer cancel to join
    // libc::pthread_join(thread2_handle, std::ptr::null_mut());
    println!("thread 1 done");
}

extern "C" fn thread_2_function(_arg: *mut c_void) -> *mut c_void {
    unsafe {
        // set current thread cancel behaviour
        pthread_setcanceltype(PTHREAD_CANCEL_ASYNCHRONOUS, std::ptr::null_mut());
        for _ in 0..3 {
            libc::usleep(50);
            println!("thread 2 print");
        }
        libc::pthread_exit(std::ptr::null_mut());
    }
}
