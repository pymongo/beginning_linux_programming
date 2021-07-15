//! ch12/thread4.c
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        main_thread();
    }
}

unsafe fn main_thread() {
    let mut mutex = std::mem::zeroed();
    let mutex_init_res = libc::pthread_mutex_init(&mut mutex, std::ptr::null());
    assert_eq!(mutex_init_res, 0);

    let mut thread_2_handle = std::mem::zeroed();
    let pthread_create_res = libc::pthread_create(
        &mut thread_2_handle,
        std::ptr::null(),
        thread_2,
        (&mut mutex as *mut libc::pthread_mutex_t).cast(),
    );
    assert_eq!(pthread_create_res, 0);

    for _ in 0..5 {
        libc::pthread_mutex_lock(&mut mutex);
        println!("thread 1 print 1");
        libc::pthread_mutex_unlock(&mut mutex);
        // delay between lock and unlock
        libc::usleep(1);
    }

    let join_res = libc::pthread_join(thread_2_handle, std::ptr::null_mut());
    assert_eq!(join_res, 0);

    libc::pthread_mutex_destroy(&mut mutex);
}

extern "C" fn thread_2(arg: *mut libc::c_void) -> *mut libc::c_void {
    let mutex = arg.cast::<libc::pthread_mutex_t>();
    for _ in 0..5 {
        unsafe {
            libc::pthread_mutex_lock(mutex);
            println!("thread 2 print 2");
            libc::pthread_mutex_unlock(mutex);
            libc::usleep(1);
        }
    }
    unsafe {
        libc::pthread_exit(std::ptr::null_mut());
    }
}
