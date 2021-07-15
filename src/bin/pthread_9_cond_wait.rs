#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        main_thread();
    }
}

struct Arg {
    cond: libc::pthread_cond_t,
    mutex: libc::pthread_mutex_t,
    condition: bool,
}

unsafe fn main_thread() {
    let mut arg = Arg {
        // libc::pthread_cond_init(&mut cond, std::ptr::null());
        cond: libc::PTHREAD_COND_INITIALIZER,
        mutex: libc::PTHREAD_MUTEX_INITIALIZER,
        condition: false,
    };

    let mut thread_2_handle = std::mem::zeroed();
    libc::pthread_create(
        &mut thread_2_handle,
        std::ptr::null(),
        thread_2,
        (&mut arg as *mut Arg).cast(),
    );
    libc::pthread_join(thread_2_handle, std::ptr::null_mut());

    libc::pthread_mutex_lock(&mut arg.mutex);
    while !arg.condition {
        libc::pthread_cond_wait(&mut arg.cond, &mut arg.mutex);
    }
    println!("thread 1 pthread_cond_wait condition to true");
    libc::pthread_mutex_unlock(&mut arg.mutex);

    libc::pthread_cond_destroy(&mut arg.cond);
    libc::pthread_mutex_destroy(&mut arg.mutex);
}

extern "C" fn thread_2(arg: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        let arg = &mut *arg.cast::<Arg>();

        libc::pthread_mutex_lock(&mut arg.mutex);
        println!("thread 2 condition false -> true");
        arg.condition = true;
        libc::pthread_cond_signal(&mut arg.cond);
        libc::pthread_mutex_unlock(&mut arg.mutex);

        libc::pthread_exit(std::ptr::null_mut());
    }
}
