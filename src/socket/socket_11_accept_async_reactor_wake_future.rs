use crate::not_minus_1;
use std::os::unix::prelude::{AsRawFd, RawFd};

struct MyTcpListener(std::net::TcpListener);

impl MyTcpListener {
    fn new(port: u16) -> Self {
        let listener = std::net::TcpListener::bind(("localhost", port)).unwrap();
        // accept would be non-blocking
        listener.set_nonblocking(true).unwrap();
        Self(listener)
    }

    const fn accept_async(&self) -> AcceptFuture {
        AcceptFuture(self)
    }
}

struct MyTask {
    future: std::sync::Mutex<Option<futures::future::BoxFuture<'static, std::io::Result<()>>>>,
    task_sender: std::sync::mpsc::SyncSender<std::sync::Arc<MyTask>>,
}

impl futures::task::ArcWake for MyTask {
    fn wake_by_ref(arc_self: &std::sync::Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("failed to send");
    }
}

/// Spawner  <-- task/future queue --> Executor(async_runtime) <-- waker --> Reactor(epoll_fd, HashMap<RawFd, Waker>)
struct Spawner {
    task_sender: std::sync::mpsc::SyncSender<std::sync::Arc<MyTask>>,
}

impl Spawner {
    fn spawn(&self, fut: impl std::future::Future<Output = std::io::Result<()>> + 'static + Send) {
        let fut = futures::FutureExt::boxed(fut);
        let task = std::sync::Arc::new(MyTask {
            future: std::sync::Mutex::new(Some(fut)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("failed to send");
    }
}

struct Executor {
    task_receiver: std::sync::mpsc::Receiver<std::sync::Arc<MyTask>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.task_receiver.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = futures::task::waker_ref(&task);
                let mut context = std::task::Context::from_waker(&*waker);
                if future.as_mut().poll(&mut context).is_pending() {
                    // 如果还没完成就重新放回队列中
                    *future_slot = Some(future);
                }
            }
        }
    }
}

struct AcceptFuture<'a>(&'a MyTcpListener);

impl<'a> std::future::Future for AcceptFuture<'a> {
    type Output = std::io::Result<std::net::TcpStream>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        println!("In `impl<'a> std::future::Future for AcceptFuture<'a>`");
        match self.0 .0.accept() {
            Ok((stream, addr)) => {
                // stream.set_nonblocking(true)?;
                println!("AcceptFuture/accept_async ok, client_addr={}", addr);
                std::task::Poll::Ready(Ok(stream))
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // 服务器刚启动第一次 poll 会是 EAGAIN 然后加到 epoll 事件中
                println!("AcceptFuture poll failed: EAGAIN\nreturn std::task::Poll::Pending");
                REACTOR.add_event((self.0).0.as_raw_fd(), libc::EPOLLIN, cx.waker().clone());
                std::task::Poll::Pending
            }
            Err(e) => std::task::Poll::Ready(Err(e)),
        }
    }
}

struct Reactor {
    epoll_fd: RawFd,
    wakers: std::sync::Mutex<std::collections::HashMap<RawFd, std::task::Waker>>,
}

/// epoll_create1 的 flag 参数: EPOLL_CLOEXEC(O_CLOEXEC) 的作用: 进行类似 exec 系统调用时关闭所有 O_CLOEXEC 标记的 fd
/// 由于我们应用没有涉及 exec 所以不需要设置该 flag
/// mio::Registry == mio/src/sys/unix/selector/epoll.rs == all epoll syscall
impl Default for Reactor {
    fn default() -> Self {
        Self {
            epoll_fd: not_minus_1!(libc::epoll_create1(0)),
            wakers: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

impl Drop for Reactor {
    fn drop(&mut self) {
        not_minus_1!(libc::close(self.epoll_fd));
    }
}

impl Reactor {
    /// private
    const fn event(fd: RawFd, event_type: libc::c_int) -> libc::epoll_event {
        libc::epoll_event {
            /* events field:
            - EPOLLIN/EPOLLOUT: read/write
            - EPOLLET(epoll edge-triggered):
                例如 nginx 会用边缘触发等内核缓冲区满才通知，避免频繁读写。
                我们应用就用默认的水平触发够了(有数据就通知)
            */
            events: event_type as u32,
            /* u64 field:
            u64 字段在 mio 中是 mio::Token 的抽象
            由于 epoll_ctl 已经有一个 fd 的入参，所以这个 u64 其实是给调用者放一些额外的信息，例如 mio::Token 抽象信息
            为了简单，我们应用就把 u64 字段
            */
            u64: fd as u64,
        }
    }

    fn add_event(&self, fd: RawFd, event_type: libc::c_int, waker: std::task::Waker) {
        not_minus_1!(libc::epoll_ctl(
            self.epoll_fd,
            libc::EPOLL_CTL_ADD,
            fd,
            &mut Self::event(fd, event_type)
        ));
        self.wakers.lock().unwrap().insert(fd, waker);
    }
}

fn reactor_main_loop() {
    let mut events = [unsafe { std::mem::zeroed() }; libc::FD_SETSIZE];
    loop {
        // epoll_wait's timeout arg -1 means to block indefinitely(no timeout)
        let events_len = not_minus_1!(libc::epoll_wait(
            REACTOR.epoll_fd,
            events.as_mut_ptr(),
            libc::FD_SETSIZE as i32,
            -1
        ));
        for event in events.iter().take(events_len as usize) {
            let fd = event.u64 as RawFd;
            if let Some(waker) = REACTOR.wakers.lock().unwrap().remove(&fd) {
                // wake a accept_future task continue poll
                waker.wake();
                // remove epoll event, 让 Future 协程下一次 poll 失败时重新插入 accept event
                not_minus_1!(libc::epoll_ctl(
                    REACTOR.epoll_fd,
                    libc::EPOLL_CTL_DEL,
                    fd,
                    std::ptr::null_mut()
                ));
            }
        }
    }
}

static REACTOR: std::lazy::SyncLazy<Reactor> = std::lazy::SyncLazy::new(|| {
    std::thread::spawn(move || {
        reactor_main_loop();
    });
    Reactor::default()
});

#[test]
fn main() {
    let (task_sender, task_receiver) = std::sync::mpsc::sync_channel(1000);
    let spawner = Spawner { task_sender };
    let executor = Executor { task_receiver };
    spawner.spawn(async {
        let listener = MyTcpListener::new(8080);
        while let Ok(stream) = listener.accept_async().await {
            let fd = stream.as_raw_fd();
            let mut buf = [0_u8; 256];
            // for simple, only accept use async future, read/write keep sync_blocking_IO
            loop {
                unsafe {
                    let n_read = libc::read(fd, buf.as_mut_ptr().cast(), buf.len());
                    if n_read == 0 {
                        libc::close(fd);
                    }
                    let n_write = libc::write(fd, buf.as_ptr().cast(), n_read as usize);
                    assert_eq!(n_read, n_write);
                }
            }
        }
        Ok(())
    });
    executor.run();
}
