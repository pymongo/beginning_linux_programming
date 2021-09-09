/*!
# **unsafe** keyword usage
1. FFI
2. from_utf8_unchecked
3. extend lifetime
4. access private field/function
5. raw pointer modified immutable var like libc::fcntl(fd, libc::F_GETLK, &region)

what can transmute do?
1. type cast
2. extend lifetime
3. access private field/function
*/

#[test]
fn unsafe_extend_lifetime() {
    let a = std::fs::read_to_string("/etc/lsb-release").unwrap();
    // let b = &a;
    let b: &'static String = unsafe { std::mem::transmute(&a) };
    let mut handles = vec![];
    for _ in 0..2 {
        handles.push(std::thread::spawn(move || {
            println!("{}", b);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    
}