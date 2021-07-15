//! ch12/thread2.c
#![warn(clippy::nursery, clippy::pedantic)]
use libc::c_void;

fn main() {
    unsafe {
        main_();
    }
}

static mut SHARED_VAR: u8 = 1;

/**
loop {
    thread_1(SHARED_VAR: 1->2)
    thread_2(SHARED_VAR: 2->1)
}
两个线程就像踢皮球那样互相占用 SHARED_VAR 有没有更好的共享可变数据的方式呢?

我的第一感觉就是用互斥锁，但要加上超时，方便调试死锁
*/
extern "C" fn thread_2_function(_arg: *mut c_void) -> *mut c_void {
    for _ in 0..10 {
        if unsafe { SHARED_VAR } == 2 {
            println!("thread_2 print 2");
            unsafe { SHARED_VAR = 1; }
        } else {
            unsafe { libc::usleep(10 * 1000); }
        }
    }
    unsafe { libc::pthread_exit(std::ptr::null_mut()); }
}

unsafe fn main_() {
    let mut thread_2 = std::mem::zeroed();
    let res = libc::pthread_create(
        &mut thread_2,
        std::ptr::null(),
        thread_2_function,
        std::ptr::null_mut(),
    );
    assert_eq!(res, 0);

    for _ in 0..10 {
        if SHARED_VAR == 1 {
            println!("thread_2 print 1");
            SHARED_VAR = 2;
        } else {
            libc::usleep(10 * 1000);
        }
    }

    let join_res = libc::pthread_join(thread_2, std::ptr::null_mut());
    assert_eq!(join_res, 0);
}
