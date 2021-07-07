#![allow(clippy::doc_markdown)]

fn main() {
    unsafe {
        assert!(std::env::var("key").is_err());
        let key = "key\0".as_ptr().cast();
        let value = "value\0".as_ptr().cast();
        let string = libc::malloc(
            libc::strlen(key) + libc::strlen("=\0".as_ptr().cast()) + libc::strlen(value) + 1,
        );
        assert!(!string.is_null());
        let string = string.cast();
        libc::strcpy(string, key);
        libc::strcat(string, "=\0".as_ptr().cast());
        libc::strcat(string, value);
        assert!(libc::putenv(string) == 0);
        let val = libc::getenv(key);
        assert!(!val.is_null());
        libc::printf("env_var key=%s\n\0".as_ptr().cast(), val);
    }
}
