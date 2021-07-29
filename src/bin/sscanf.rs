use beginning_linux_programming::syscall;

fn main() {
    let mut a = 0_i32;
    let mut b = 0_i32;
    let modified_count1 = syscall!(sscanf(
        " a  1     2\0".as_ptr().cast(),
        " a %d %d\0".as_ptr().cast(),
        &mut a,
        &mut b,
    ));
    assert_eq!(modified_count1, 2);
    assert_eq!(a, 1);

    let mut int32 = 0_i32;
    // %g is map to f32
    let mut float32 = 0_f32;
    let mut char_ = 0_u8;
    // %[^,] map a string stop at first comma(not include comma)
    // %s can't include space, %[] match string can include white space
    let mut s = [0_u8; 128];
    let modified_count2 = syscall!(sscanf(
        "123 4.56 C My name is apple, my age is 18.\0"
            .as_ptr()
            .cast(),
        "%d %g %c %[^,]\0".as_ptr().cast(),
        &mut int32,
        &mut float32,
        &mut char_,
        &mut s,
    ));
    assert_eq!(modified_count2, 4);
    assert!((float32 - 4.56).abs() < f32::EPSILON);
    assert_eq!(char_, b'C');
    assert_eq!(
        unsafe { String::from_utf8_unchecked(s[.."My name is apple".len()].to_vec()) },
        "My name is apple"
    );
    println!("ok");
}
