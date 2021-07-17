#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    unsafe {
        run(true);
    }
}

#[test]
fn test_sender() {
    unsafe {
        run(false);
    }
}

// /usr/include/linux/msg.h
// MSGMAX: max size in bytes in a message
// const MSGMAX: usize = 8192;
// MSGMNB: max number of messages in a message queue
// const MSGMNB: u32 = 16384;

/// 注意结构体第一个字段必须是 `c_long` 这是个内存不安全的示例
#[allow(dead_code)]
#[derive(Debug)]
enum RequestMessage {
    Unknown,
    SignIn,
    SignUp,
}

unsafe fn run(is_receiver: bool) {
    let msg_size = std::mem::size_of::<RequestMessage>();
    let msqid = libc::msgget(123, 0o666 | libc::IPC_CREAT);

    if is_receiver {
        let mut recv_data: RequestMessage = std::mem::zeroed();
        // copy data from mq to recv_data
        let res = libc::msgrcv(
            msqid,
            (&mut recv_data as *mut RequestMessage).cast::<libc::c_void>(),
            msg_size,
            0,
            0,
        );
        if res == -1 {
            libc::perror(std::ptr::null_mut());
            libc::exit(libc::EXIT_FAILURE);
        }
        dbg!(recv_data);
    } else {
        let req_msg = RequestMessage::SignUp;
        // copy data to mq from req_msg
        let res = libc::msgsnd(
            msqid,
            (&req_msg as *const RequestMessage).cast::<libc::c_void>(),
            msg_size,
            0,
        );
        if res == -1 {
            libc::perror(std::ptr::null_mut());
        }
    }

    libc::msgctl(msqid, libc::IPC_RMID, std::ptr::null_mut());
}
