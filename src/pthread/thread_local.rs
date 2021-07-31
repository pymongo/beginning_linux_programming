use std::cell::{Cell, RefCell};

struct MySafeString(RefCell<String>);

// 欺骗编译器我这个类型是安全的，只要我使用前从环境变量中读取值并进行初始化，初始化后值再也不会修改，所以是多线程安全的
unsafe impl Sync for MySafeString {}

static MARKET_ID: MySafeString = MySafeString(RefCell::new(String::new()));

#[test]
fn main() {
    println!("{}", MARKET_ID.0.borrow());
    *MARKET_ID.0.borrow_mut() = "1".to_string();
    assert_eq!("1", *MARKET_ID.0.borrow());
    println!("{}", MARKET_ID.0.borrow());
    // 为了让 thread_local 的 static 变量具有可变性，建议用 Cell 或 RefCell，由于这里不希望 u32 被 Copy，所以用 RefCell
    // thead_local 一般搭配Cell和RefCell一起使用，可以定义线程独享数据
    thread_local! {static DATA: Cell<u32> = Cell::new(1)};
    std::thread::spawn(|| {
        DATA.with(|f| {
            assert_eq!(f.get(), 1);
            // 在线程 2 中将 DATA 改为 2
            f.set(2);
        });
    });
    // DATA的类型: std::thread::LocalKey<std::cell::Cell<u32>>
    DATA.with(|f| {
        // 在 main 线程中 DATA 还是 1
        assert_eq!(f.get(), 1);
    });
}
