struct MyCell<T> {
    value: T,
}
impl<T: Copy> MyCell<T> {
    fn new(x: T) -> MyCell<T> {
        MyCell { value: x }
    }
    fn set(&self, value: T) {
        use std::ptr;
        unsafe {
            ptr::write(&self.value as *const T as *mut T, value);
        }
    }
}

fn step1<'a>(r_c1: &MyCell<&'a i32>) {
    let val: i32 = 13;
    step2(&val, r_c1); // step2函数执行完再回到step1
    println!("step1 value: {}", r_c1.value);
} // step1调用完，栈帧将被清理，val将不复存在，&val将成为悬垂指针

// a': 'b, 'b的生存期小于'a, 'a是'b的子类型, 由于默认协变所以'b处可以传入'a
fn step2<'b>(r_val: &'b i32, r_c2: &MyCell<&'b i32>) {
    r_c2.set(r_val);
}
static X: i32 = 10;
fn main() {
    let cell = MyCell::new(&X);
    step1(&cell);
    println!("  end value: {}", cell.value); //此处 cell.value的值将无法预期，UB风险
}
