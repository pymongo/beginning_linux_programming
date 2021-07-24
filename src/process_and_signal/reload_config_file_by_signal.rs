//! 严格来说 hot-reload 指的是 nodejs 的 grub 这样的工具，例如 docsify 改一行 html 源码，网页就立即刷新。我这个动态加载配置文件並不算 hot-reload
#![warn(clippy::nursery, clippy::pedantic)]

#[derive(Debug)]
pub struct Config {
    aws_s3_key: String,
}

impl Config {
    pub fn load_production_config() -> Self {
        let config_str =
            std::fs::read_to_string("src/process_and_signal/config_file.toml").unwrap();
        dbg!(&config_str);
        let key = config_str.trim().split_once('=').unwrap().1;
        Self {
            aws_s3_key: key.to_string(),
        }
    }
}

enum Command {
    ReloadConfigFileFromDisk,
    PrintConfigFile,
}

static mut LAST_COMMAND: Command = Command::PrintConfigFile;

fn siguser1_callback(_sig: i32) {
    unsafe {
        LAST_COMMAND = Command::ReloadConfigFileFromDisk;
    }
}

fn siguser2_callback(_sig: i32) {
    unsafe {
        LAST_COMMAND = Command::PrintConfigFile;
    }
}

/// alternative: polling or inotify - monitoring filesystem events
#[test]
fn main() {
    let mut config = Config::load_production_config();
    // set signal callback handler
    unsafe {
        dbg!(libc::gettid()); // get pid to send signal
        libc::signal(libc::SIGUSR1, siguser1_callback as libc::sighandler_t);
        libc::signal(libc::SIGUSR2, siguser2_callback as libc::sighandler_t);
    }
    loop {
        match unsafe { &LAST_COMMAND } {
            Command::ReloadConfigFileFromDisk => {
                config = Config::load_production_config();
                println!("config reload from disk success");
            }
            Command::PrintConfigFile => {
                dbg!(&config);
            }
        }

        // suspend thread
        unsafe {
            libc::pause();
        }
    }
}
