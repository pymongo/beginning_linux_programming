//! ch12/thread3.c

#[test]
fn main() {
    unsafe {
        main_thread();
    }
}

unsafe fn main_thread() {
    // here mutex aka binary semaphore
    let mut mutex = std::mem::zeroed();
    let mutex_init_res = libc::sem_init(&mut mutex, 0, 0);
    assert_eq!(mutex_init_res, 0);

    let mut thread_2_handle = std::mem::zeroed();
    let pthread_create_res = libc::pthread_create(
        &mut thread_2_handle,
        std::ptr::null(),
        thread_2,
        (&mut mutex as *mut libc::sem_t).cast(),
    );
    assert_eq!(pthread_create_res, 0);

    for _ in 0..5 {
        println!("thread 1 print 1");
        // release semaphore
        libc::sem_post(&mut mutex);
        libc::usleep(1);
    }

    let join_res = libc::pthread_join(thread_2_handle, std::ptr::null_mut());
    assert_eq!(join_res, 0);

    libc::sem_destroy(&mut mutex);
}

extern "C" fn thread_2(arg: *mut libc::c_void) -> *mut libc::c_void {
    let mutex = arg.cast::<libc::sem_t>();
    for _ in 0..5 {
        unsafe {
            libc::sem_wait(mutex);
            println!("thread 2 print 2");
            libc::usleep(1);
        }
    }
    unsafe {
        libc::pthread_exit(std::ptr::null_mut());
    }
}
