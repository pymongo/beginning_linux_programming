use super::reload_config_file_by_signal::Config;
use crate::syscall;
use libc::inotify_event;

const MAX_EVENTS: usize = 32;

#[test]
fn main() {
    let mut config = Config::load_production_config();
    dbg!(&config);
    let inotify_fd = syscall!(inotify_init());
    let config_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/process_and_signal/config_file.toml\0"
    );
    let _wd = syscall!(inotify_add_watch(
        inotify_fd,
        config_path.as_ptr().cast(),
        libc::IN_MODIFY
    ));
    let mut events: [inotify_event; MAX_EVENTS];
    loop {
        events = unsafe { std::mem::zeroed() };
        let events_len = syscall!(read(
            inotify_fd,
            (&mut events as *mut inotify_event).cast(),
            MAX_EVENTS * std::mem::size_of::<inotify_event>()
        ));
        println!("after read");
        for event in events.iter().take(events_len as usize) {
            if event.mask & libc::IN_MODIFY == libc::IN_MODIFY {
                syscall!(usleep(10 * 1000));
                config = Config::load_production_config();
                dbg!(&config);
            }
        }
    }
    // inotify_rm_watch(inotify_fd, wd);
    // close(inotify_fd);
}
