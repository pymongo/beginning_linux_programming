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
