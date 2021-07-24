/*!
# 异步编程笔记

## task 模型

- Future: 基本的异步计算抽象单元
- Task: 异步计算的执行单元
- Executor: 异步计算调度曾

### 如何定义 runtime/Executor (task 和 runtime 的关系)

调度协程(task)的就叫异步运行时，异步运行时也叫 runtime/executor/worker_thread?

生产环境一般都是 N:M 的绿色线程/协程模型: 也就是 (N<M) N 个物理线程内 运行 M 个 task 协程

### executor 如何调度协程

#### golang 有栈协程抢占式调度

通过定时器 run_inverval 发特定信号，例如 SIGURG

确保调度器「能按时间片均匀介入」

有栈协程功能强，使用简单，缺点性能差点。无栈协程虽然弱了点，但也够用

### task 和 future (task 模型)

一个协程(task)里面可以运行一个或多个 future

### leaf-futures and non-leaf-futures
non-leaf-futures 一般是 async/await 创建的，不那么底层(业务层)，没有跟异步框架底层 reactor 直接打交道

为什么会有这个概念——因为 task 内部也可以 spawn 一个 task

#### 叶子 future

例如 AcceptFuture 它是最底层的 Future 直接跟 Reactor(监听 epoll 事件并唤醒能处理相应事件的 task) 通信的

### 如何创建 task 协程

一般通过 Spawner::spawn

### await 和 Reactor
await pending 时 yield 协程，reactor 通过 epoll 发现数据已经准备好了再去唤醒 await，程序继续往下执行

### **Java Netty/NIO 的 reactor**

---

## waker 唤醒机制

### std::task::Context

为什么 task 会有 context 上下文: 因为 无栈协程是个状态机，每次切换调度是都需要存储自身状态

```rust
struct Context<'a> {
    waker: &'a Waker,
    // Ensure we future-proof against variance changes by forcing
    // the lifetime to be invariant (argument-position lifetimes are contravariant while return-position lifetimes are covariant).
    // 强制把入参引用的型变检查设置成「不变」
    _marker: PhantomData<fn(&'a ()) -> &'a ()>,
}
```

### std::task::Waker(std::task::RawWaker)

reactor 通知 future_context_waker 客户端连接的 socket 有数据啦

future_context_waker 再去通知 executor/runtime 去唤醒执行该 future 的 task 协程

RawWaker 的内存布局跟 trait_object 类似，相当于模拟了一个 trait_object

```rust
pub struct RawWaker {
    /// A data pointer, which can be used to store arbitrary data as required
    /// by the executor. This could be e.g. a type-erased pointer to an `Arc`
    /// that is associated with the task.
    /// The value of this field gets passed to all functions that are part of
    /// the vtable as the first parameter.
    /// data 可以用来存放 executor 需要的一些数据
    data: *const (),
    /// Virtual function pointer table that customizes the behavior of this waker.
    /// vtable 保存了唤醒的机制和行为(如何去唤醒)
    vtable: &'static RawWakerVTable,
}
```

> RawWaker 为什么要实现一个 vtable? 为什么没有用类似效果的 trait_object 呢?

因为 RawWaker 还要实现 Clone，为了引入并发性同时等待多个事件，所以一个 waker 不能只被一个 事件源拥有所有权，所以需要 clone

### trait std::task::Wake

### wake_by_ref

通过引用去 wake 避免消耗掉 waker

### wake 虚表的方法
- clone
- wake
- wake_by_ref
- drop

---

## futures-executor

有两种，一种是单线程的，叫 LocalSpawner 另一种则是多线程的叫 thread-poll

### LocalSpawner

LocalSpawner 维护了一个队列(Vec<Future>)

还有一个重要概念就是协程通知相关的 ThreadNotify 结构体 park/unpark 的布尔值

单线程也用 Arc<ThreadNotify> 的原因是编译器限制，调用者能保证单线程，但是 static 的变量编译器不保证一定只有一个线程

Atomic unpark 设置成 true 就表示唤醒，因为是单线程，所以所有 unpark AtomicBool 的操作都用 Relaxed 的读写顺序

生产环境下的运行时不会让线程 park 进入 sleep 状态，futures-rs 这个没有用 epoll 之类的通知所以只能自己维护通知机制

futures 的多线程运行时没有绑定 epoll 基于 park/unpark 调度，poll 会有惊群问题导致数据的线程安全问题

futures 调度比较简单，poll Pending则再发到任务队列中，所以 futures 不像一个很完整的异步运行时

---

## await 语法糖

async 比较好理解展开成 impl Future<Output=xxx>

await的话会暂停在这行直到异步运行时把该 Future poll 到 Ready 的状态

await 展开后的代码可以参看老版 await! 宏，也就 loop poll，pending 时就 yield 协程对 CPU 的占用让 executor 调度执行其他 Future

老版标准库的 await 宏

```text
macro_rules! await_old {
    ($e:expr) => {{
        let mut pinned = $e;
        let mut pinned = unsafe { $crate::mem::PinMut::new_unchecked(&mut pinned) };
        loop {
            match $crate::future::poll_in_task_cx(&mut pinned) {
                $crate::task::Poll::Pending => yield,
                $crate::task::Poll::Ready(x) => break x,
            }
        }
    }}
}
```

### 标准库的 GenFuture 生成器输出 Future

- yield -> pending
- complete -> ready

生成器只能和线程互相调度，并不能实现生成器互相调度，所以光生成器只能是个「半步协程」

生成器还需要配合 reactor, executor 等等才能算是真正的用户态自我调度协程

## 工作窃取调度算法:

空闲且队列空的线程会去其他线程任务队列尾部偷一个任务回来执行

===

# Pin

主要为了编译时检查，没有 Pin 就跟 C++ 一样程序员肉眼 review 内存安全代码

## 生成器的自引用

例如生成器内定义了一个字符串，但是把字符串引用 yield 给其它协程了，这时候来回多次传递字符串引用

最终生成器容易有 UB 的安全问题，但是为了性能，是一定会把指向生成器内部数据的引用 yield 传递出去

在 Safe Rust 下没法创建「自引用」结构体

但是自引用指向的数据出现 move 或 swap 之后，就会 segfault 内存错误

> impl<T: ...> !Unpin for GenFuture<T>

Pin 住以后，如果是 `!Unpin` 则不能拿到可变引用，标准库大部分类型都自动实现 Unpin

GenFuture 没有实现 Unpin，所以自引用是安全的，也就是 Pin 智能指针结构体要结合 `!Unpin` 的数据才能实现不可移动的效果

一般通过 PhantomPin 给自己结构体加上 `!Unpin`

## &str 和 &String

&str 一般是指向静态存储区 static 字符串的字面量，但 String::as_str 拿到的指针会在堆上

例如 &String 的地址是在当前栈帧，而 String::as_str() 的地址会在堆内存上

Pin 到栈上的指针会跟随函数调用栈帧变化而变化，不如堆上的稳定，所以 !Unpin 在栈上指针需要 Unsafe
*/
use crate::syscall;
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

/// 一个协程(task)里面可以运行一个或多个 future
/// 我们的 accept_async 代码都是 一个 task 对应一个 future，先学会这种比较简单的
struct MyTask {
    future: std::sync::Mutex<Option<futures::future::BoxFuture<'static, std::io::Result<()>>>>,
    task_sender: std::sync::mpsc::SyncSender<std::sync::Arc<MyTask>>,
}

impl futures::task::ArcWake for MyTask {
    /// reactor 的 epoll 发现有客户端数据时，MyTask 重新加入到任务队列
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
    /// Input: Future, Output: Task
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
            epoll_fd: syscall!(epoll_create1(0)),
            wakers: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

impl Drop for Reactor {
    fn drop(&mut self) {
        syscall!(close(self.epoll_fd));
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
        syscall!(epoll_ctl(
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
        let events_len = syscall!(epoll_wait(
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
                syscall!(epoll_ctl(
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
#[ignore = "must run both server and client"]
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
