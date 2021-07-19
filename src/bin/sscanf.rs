fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    dbg!(args);
    unsafe {
        main_();
    }
}

unsafe fn main_() {
    let mut a = 0_i32;
    let mut b = 0_i32;
    let modified_count1 = libc::sscanf(
        " a  1     2\0".as_ptr().cast(),
        " a %d %d\0".as_ptr().cast(),
        &mut a,
        &mut b,
    );
    assert_eq!(modified_count1, 2);
    assert_eq!(a, 1);

    let mut int32 = 0_i32;
    // %g is map to f32
    let mut float32 = 0_f32;
    let mut char_ = 0_u8;
    // %[^,] map a string stop at first comma(not include comma)
    // %s can't include space, %[] match string can include white space
    let mut s = [0_u8; 128];
    let modified_count2 = libc::sscanf(
        "123 4.56 C My name is apple, my age is 18.\0"
            .as_ptr()
            .cast(),
        "%d %g %c %[^,]\0".as_ptr().cast(),
        &mut int32,
        &mut float32,
        &mut char_,
        s.as_mut_ptr(),
    );
    assert_eq!(modified_count2, 4);
    assert!((float32 - 4.56).abs() < f32::EPSILON);
    assert_eq!(char_, b'C');
    assert_eq!(
        String::from_utf8_unchecked(s[.."My name is apple".len()].to_vec()),
        "My name is apple"
    );
}
