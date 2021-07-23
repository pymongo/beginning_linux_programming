#[test]
fn main() {
    unsafe {
        main_();
    }
}

const PASSWORD_LEN: usize = 6;

unsafe fn main_() {
    let mut old_termios = std::mem::zeroed();
    libc::tcgetattr(libc::STDIN_FILENO, &mut old_termios);

    let mut hide_echo_setting = old_termios;
    hide_echo_setting.c_lflag &= !libc::ECHO;

    print!("Enter password: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    // libc::TCSAFLUSH previous input before change termios setting
    assert_eq!(
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &hide_echo_setting),
        0
    );

    let mut password = [0_u8; PASSWORD_LEN + 1];
    let stdin = libc::fdopen(libc::STDIN_FILENO, "r\0".as_ptr().cast());
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    libc::fgets(password.as_mut_ptr().cast(), PASSWORD_LEN as i32, stdin);
    libc::fclose(stdin);
    libc::printf(
        "\nYour entered %s\n\0".as_ptr().cast(),
        password.as_ptr().cast::<libc::c_char>(),
    );

    // same as `stty echo` to enable echo
    libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &old_termios);
}
