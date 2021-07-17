#![warn(clippy::nursery, clippy::pedantic)]

#[link(name = "c")]
extern "C" {
    /// same as `asctime(localtime(time_t))`
    fn ctime(timestamp: *const libc::time_t) -> *const libc::c_char;
}

fn main() {
    let is_receiver = std::env::args().len() == 1;
    unsafe {
        run(is_receiver);
    }
}

#[test]
fn test_sender() {
    unsafe {
        run(false);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
enum Request {
    /// unreachable, 因为接收时用 mem::zero 创建默认的 Message 时，就设成 Unknown
    Unknown,
    Join,
    Leave,
}

#[derive(Debug)]
#[repr(C)]
struct Message {
    chat_room_id: libc::c_long,
    request: Request,
}

unsafe fn print_mq_status(msqid: i32) {
    // IPC_SET 命令是修改 MQ 的 msqid_ds 状态结构体
    let mut msqid_ds = std::mem::zeroed();
    let res = libc::msgctl(msqid, libc::IPC_STAT, &mut msqid_ds);
    if res != 0 {
        panic!("{}", std::io::Error::last_os_error());
    }
    libc::printf(
        "msqid_ds.msg_rtime(receive time) = %s\0".as_ptr().cast(),
        ctime(&msqid_ds.msg_rtime),
    );
    libc::printf(
        "msqid_ds.msg_stime = %s\0".as_ptr().cast(),
        ctime(&msqid_ds.msg_stime),
    );
    libc::printf(
        "msqid_ds.msg_qnum(num msgs in MQ) = %d\n\0".as_ptr().cast(),
        msqid_ds.msg_qnum,
    );
    libc::printf(
        "msqid_ds.msg_lspid(sender PID) = %d\n\0".as_ptr().cast(),
        msqid_ds.msg_lspid,
    );
    libc::printf(
        "msqid_ds.msg_lrpid(receiver PID) = %d\n\0".as_ptr().cast(),
        msqid_ds.msg_lrpid,
    );
    // libc::printf(
    //     "msqid_ds.msg_perm = %o\n\0".as_ptr().cast(),
    //     msqid_ds.msg_perm,
    // );
    println!();
}

unsafe fn run(is_receiver: bool) {
    let msg_size = std::mem::size_of::<Message>() - std::mem::size_of::<libc::c_long>();
    let msqid = libc::msgget(12, 0o666 | libc::IPC_CREAT);

    if is_receiver {
        let mut recv_data: Message = std::mem::zeroed();
        /* msgrcv: 读取/消费 System V 消息队列 消息
        注意 send/receive 的 msgsz 都要去掉消息结构体的第一个字段也就是 c_long 的大小
        msgtyp 表示 「接收优先级」(reception priority)
        配合 msgtype 是负数时例如 -3 会消费第一条优先级 <=3 的消息
        其实不是过滤，只是优先度低的还在队列中，前面高优先级的都跳过
        msgtype=2 表示找到消息队列中第一条优先级等于 2 的消息并消费掉
        */
        let res = libc::msgrcv(
            msqid,
            (&mut recv_data as *mut Message).cast::<libc::c_void>(),
            msg_size,
            // only receive chat_room_id=2 message
            // if msgtype == 0, receive all message type
            2,
            // 0 表示不设置 IPC_NOWAIT
            // 发送端默认当队列满时候 suspend 发送进程
            // 接收端默认当队列空时候 suspend 接收进程
            0,
        );
        if res == -1 {
            // libc::perror(std::ptr::null_mut());
            // libc::exit(libc::EXIT_FAILURE);
            panic!("{}", std::io::Error::last_os_error());
        }
        dbg!(recv_data);
        // receiver 进程没有读取 MQ 状态的权限? EINVAL?
        print_mq_status(msqid); // msgtcl panic
    } else {
        for chat_room_id in 1..=2 {
            let req_msg = Message {
                chat_room_id,
                request: Request::Join,
            };
            // POSIX's mq: libc::mq_send()
            let res = libc::msgsnd(
                msqid,
                (&req_msg as *const Message).cast::<libc::c_void>(),
                msg_size,
                0,
            );
            if res == -1 {
                panic!("{}", std::io::Error::last_os_error());
            }
            print_mq_status(msqid);
        }
    }
    // print_mq_status(msqid);
    libc::msgctl(msqid, libc::IPC_RMID, std::ptr::null_mut());
}
