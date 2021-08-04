use crate::syscall;

#[link(name = "c")]
extern "C" {
    type semid_ds;
    type seminfo;
}

#[allow(non_camel_case_types)]
union semun {
    val: i32,
    #[allow(dead_code)]
    buf: *mut semid_ds,
    #[allow(dead_code)]
    array: *mut u16,
    #[allow(dead_code)]
    seminfo: *mut seminfo,
}

enum SemOp {
    Passeren = -1,
    Vrijgeven = 1,
}

const SEM_UNDO: libc::c_short = 0x1000;

fn pv(sem_id: i32, sem_op: SemOp) {
    let mut sem_buf = libc::sembuf {
        sem_num: 0,
        sem_op: sem_op as i16,
        sem_flg: SEM_UNDO,
    };
    syscall!(semop(sem_id, &mut sem_buf, 1));
}

const SETVAL: i32 = 16;

#[test]
fn main() {
    let sem_id = syscall!(semget(libc::IPC_PRIVATE, 1, 0o644));
    let sem_un = semun { val: 1 };
    // sem_num 参数指定被操作的信号量在信号量集中的编号
    syscall!(semctl(sem_id, 0, SETVAL, sem_un));

    let pid = syscall!(fork());
    if pid == 0 {
        println!("child try to lock binary sem");
        pv(sem_id, SemOp::Passeren);
        println!("child get sem would release in 1 second...");
        unsafe { libc::sleep(1) };
        pv(sem_id, SemOp::Vrijgeven);
        unsafe { libc::exit(libc::EXIT_SUCCESS) };
    }

    println!("parent try to lock binary sem");
    pv(sem_id, SemOp::Passeren);
    println!("parent get sem would release in 1 second...");
    unsafe { libc::sleep(1) };
    pv(sem_id, SemOp::Vrijgeven);

    syscall!(waitpid(pid, std::ptr::null_mut(), 0));
    let sem_un = semun { val: 0 };
    syscall!(semctl(sem_id, 0, libc::IPC_RMID, sem_un));
}
