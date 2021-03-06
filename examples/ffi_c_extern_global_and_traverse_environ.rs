#![warn(clippy::nursery, clippy::pedantic)]
#[link(name = "c")]
extern "C" {
    /// extern char **environ;
    /// In C, `extern` mean `public static`, `static` mean `private static`
    static environ: *const *const libc::c_char;
}

#[link(name = "readline")]
extern "C" {
    static rl_readline_version: libc::c_int;
}

unsafe fn traverse_env_vars() {
    // for (key, value) in std::env::vars()
    let mut env_vars = environ;
    while !(*env_vars).is_null() {
        libc::printf("%s\n\0".as_ptr().cast(), *env_vars);
        env_vars = env_vars.offset(1);
    }
}

fn main() {
    println!("version of `libreadline.so` = {}", unsafe {
        rl_readline_version
    });
    unsafe {
        traverse_env_vars();
    }
}
