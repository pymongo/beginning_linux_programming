#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]

fn main() {
    unsafe {
        set_lock();
    }
}

const FILENAME: *const libc::c_char = "/tmp/my_cargo.lock\0".as_ptr().cast();
const DEFAULT_BYTE: u8 = b'a';
const BYTES_LEN: libc::c_long = 4;

unsafe fn set_lock() {
    let fd = libc::open(
        FILENAME,
        libc::O_RDWR | libc::O_CREAT,
        libc::S_IRUSR | libc::S_IWUSR,
    );
    if fd == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }
    // write 10 bytes to file
    for _ in 0..BYTES_LEN {
        libc::write(fd, (DEFAULT_BYTE as *const u8).cast(), 1);
    }
    let current_pid = libc::getpid();
    let shared_immutable_region = libc::flock {
        l_type: libc::F_RDLCK as i16,
        l_whence: libc::SEEK_SET as i16,
        l_start: 0,
        l_len: 1,
        l_pid: current_pid,
    };
    let exclusive_mutable_region = libc::flock {
        l_type: libc::F_WRLCK as i16,
        l_whence: libc::SEEK_SET as i16,
        l_start: 1,
        l_len: 1,
        l_pid: current_pid,
    };
    // F_SETLKW 的唯一区别就是会像自旋锁那样获取失败时不断轮询阻塞线程，直到获取成功，所以获取失败时也不会返回错误码(容易死锁)，W就是wait的意思
    let ret1 = libc::fcntl(fd, libc::F_SETLK, &shared_immutable_region);
    if ret1 == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }
    let ret2 = libc::fcntl(fd, libc::F_SETLK, &exclusive_mutable_region);
    if ret2 == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }
    println!("process {} is locking the file", libc::getpid());
    libc::sleep(12);
    libc::close(fd);
    libc::unlink(FILENAME);
}

#[cfg(test)]
const fn new_flock(start_addr: libc::c_long) -> libc::flock {
    libc::flock {
        l_type: libc::F_WRLCK as i16,
        l_whence: libc::SEEK_SET as i16,
        l_start: start_addr,
        l_len: 1,
        l_pid: -1,
    }
}

#[cfg(test)]
unsafe fn get_lock() {
    let fd = libc::open(
        FILENAME,
        libc::O_RDWR | libc::O_CREAT,
        libc::S_IRUSR | libc::S_IWUSR,
    );
    if fd == -1 {
        panic!(
            "Please run set_lock() first: {}",
            std::io::Error::last_os_error()
        );
    }
    // let current_pid = libc::getpid();
    for start_addr in 0..BYTES_LEN {
        // test acquire lock of region
        // same struct flock address can't reuse in multi F_GETLK call(because after call may rewrite all fields)
        // 由于F_GETLK命令可能修改掉flock结构体的所有字段，例如请求WLOCK[10,15]，返回值会告诉你当前文件的[10,20]被WLOCK,所以l_len等字段被修改
        let mut check_wlock_region = new_flock(start_addr);
        // 这里传入&region也行，这就是「unsafe的一种用法」修改不可变变量的值
        let ret = libc::fcntl(fd, libc::F_GETLK, &mut check_wlock_region);
        if ret == -1 {
            panic!("{}", std::io::Error::last_os_error());
        }
        let lock_status = match i32::from(check_wlock_region.l_type) {
            libc::F_RDLCK => "F_RDLCK",
            libc::F_WRLCK => "F_WRLCK",
            libc::F_UNLCK => "F_UNLCK",
            _ => unreachable!(),
        };
        println!("byte_index {} is {}", start_addr, lock_status);
        if check_wlock_region.l_pid != -1 {
            println!("acquire rlock or wlock would failed");
            continue;
        }
    }
    libc::close(fd);
    libc::unlink(FILENAME);
}

#[test]
fn test_get_lock() {
    unsafe {
        get_lock();
    }
}
