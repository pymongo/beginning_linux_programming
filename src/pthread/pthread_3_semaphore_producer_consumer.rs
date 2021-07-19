//! ch12/thread3.c
use libc::{c_void, sem_t};

// static mut BINARY_SEMAPHORE: libc::sem_t = unsafe { std::mem::zeroed() };
#[test] // FIXME process hang
fn main() {
    unsafe {
        producer();
    }
}

/// 偷懒一点，不把信号量和生产者创建的数据包成一个结构体转成 void* 发给消费者线程
static mut DATA: u8 = 0;

unsafe fn producer() {
    let mut binary_semaphore = std::mem::zeroed();
    libc::sem_init(&mut binary_semaphore, 0, 0);

    let mut thread_2 = std::mem::zeroed();
    let res = libc::pthread_create(
        &mut thread_2,
        std::ptr::null(),
        consumer,
        (&mut binary_semaphore as *mut sem_t).cast(),
    );
    assert_eq!(res, 0);

    for _ in 0..5 {
        println!("producer thread send DATA={}", DATA);
        // release the semaphore to thread2
        libc::sem_post(&mut binary_semaphore);
        // block producer wait user_input to increase data
        // !NOTE please press enter to input a single char
        libc::getchar();
        DATA += 1;
    }

    let join_res = libc::pthread_join(thread_2, std::ptr::null_mut());
    assert_eq!(join_res, 0);

    libc::sem_destroy(&mut binary_semaphore);
}

extern "C" fn consumer(arg: *mut c_void) -> *mut c_void {
    let binary_semaphore = arg.cast::<sem_t>();
    // waite thread 1 print
    unsafe {
        libc::sem_wait(binary_semaphore);
    }
    loop {
        println!("consumer thread receive {}", unsafe { DATA });
        if unsafe { DATA } >= 4 {
            break;
        }
        unsafe {
            libc::sem_wait(binary_semaphore);
        }
    }
    unsafe {
        libc::pthread_exit(std::ptr::null_mut());
    }
}
