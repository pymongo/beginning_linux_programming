//! ch12/thread6.c
#![warn(clippy::nursery, clippy::pedantic)]
use libc::c_void;

/// 由于是更早版本的 Linux 没有这个 API, 为了兼容老的 Linux 机器能编译 libc ，所以没加这个 API
#[link(name = "pthread")]
extern "C" {
    fn pthread_attr_setschedpolicy(
        attr: *mut libc::pthread_attr_t,
        policy: libc::c_int,
    ) -> libc::c_int;
}

fn main() {
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let min_priority = libc::sched_get_priority_min(libc::SCHED_OTHER);
    let max_priority = libc::sched_get_priority_max(libc::SCHED_OTHER);
    dbg!(min_priority, max_priority);
    let mut sched_param: libc::sched_param = std::mem::zeroed();
    sched_param.sched_priority = min_priority;

    let mut thread_attr = std::mem::zeroed();
    assert_eq!(libc::pthread_attr_init(&mut thread_attr), 0);
    pthread_attr_setschedpolicy(&mut thread_attr, min_priority);

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
