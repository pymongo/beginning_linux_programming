#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        run(true);
    }
}

#[test]
fn producer() {
    unsafe {
        run(false);
    }
}

unsafe fn run(is_consumer: bool) {
    let mut is_running = true;

    /* libc::shmget: create shared memory
     * arg_1-key: System V IPC key
     * arg_2-size: shared memory len in bytes
     * arg_3-shmflag: read/write permission bits to shared_memory, flag usually `0666 | IPC_CREAT`
     * return: shmid of shared_memory
     */
    let shmid = libc::shmget(1234, std::mem::size_of::<bool>(), 0o622 | libc::IPC_CREAT);
    assert_ne!(shmid, -1);

    /* libc::shmat: attach libc::shmget created/return shmid to current process
    * arg_1-shmid: libc::shmget's return
    * arg_2-shmaddr: 0 is process automatic alloc mem addr, like socket 0 is random port alloc by system
    * arg_3-shmflg: SHM_RND, SHM_RDONLY
        because our shmaddr is NULL system auto alloc mem addr, you can't use SHM_RND to gen random addr
        we want other processes has write permission, so SHM_RDONLY is not set
    * return: shmaddr, first byte address of shared_memory
    */
    let shmaddr = libc::shmat(shmid, std::ptr::null(), 0);
    assert_ne!(shmaddr, (-1_isize as *mut isize).cast::<libc::c_void>());

    // consumer: mem false -> true
    // producer: mem true -> false
    let mem = shmaddr.cast::<bool>();
    let mut modified_count = 0;
    while is_running {
        if is_consumer {
            if !*mem {
                *mem = true;
                println!("consumer: false -> true");
                modified_count += 1;
            }
        } else if *mem {
            *mem = false;
            println!("producer: true -> false");
            modified_count += 1;
        }
        if modified_count >= 5 {
            is_running = false;
        }
    }

    /* libc::shmdt: detach libc::shmat's return shmaddr to current process
     * arg_1-shmaddr: libc::shmaddr's return
     * return: -1 if failed
     */
    let shm_detach_res = libc::shmdt(shmaddr);
    assert_ne!(shm_detach_res, -1);

    /* libc::shmctl: similar to libc::semctl
     * arg_1-shmid: libc::shmget's return
     * arg_2-command: IPC_RMID or IPC_STAT or IPC_SET
     * arg_3(struct shmid_id*)-buf: a pointer to the structure containing the modes and permissions for the shared memory, in our program is null
     * return: -1 if failed, the second process try to IPC_RMID must failed, should we doesn't check it return val
     */
    libc::shmctl(shmid, libc::IPC_RMID, std::ptr::null_mut());
}
