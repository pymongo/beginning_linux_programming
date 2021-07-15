//! ch12/thread5.c
#![warn(clippy::nursery, clippy::pedantic)]
use libc::c_void;

fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let mut thread_attr = std::mem::zeroed();
    assert_eq!(libc::pthread_attr_init(&mut thread_attr), 0);
    assert_eq!(
        libc::pthread_attr_setdetachstate(&mut thread_attr, libc::PTHREAD_CREATE_DETACHED),
        0
    );

    let mut thread2_handle = std::mem::zeroed();
    // pthread_create handle can't use null ptr
    libc::pthread_create(
        &mut thread2_handle,
        &thread_attr,
        thread_2_function,
        std::ptr::null_mut(),
    );

    libc::pthread_attr_destroy(&mut thread_attr);
    // wait thread 2 done
    libc::usleep(100);
    println!("thread 1 done");
}

extern "C" fn thread_2_function(_arg: *mut c_void) -> *mut c_void {
    unsafe {
        libc::usleep(50);
        println!("thread 2 done");
        libc::pthread_exit(std::ptr::null_mut());
    }
}
