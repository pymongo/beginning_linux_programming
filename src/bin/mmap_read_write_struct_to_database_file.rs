#![warn(clippy::nursery, clippy::pedantic)]

type Username = [u8; 7];

/// 只要结构体的各个字段都是栈上内存，没有指针，就无需序列化也能读写进文件中
#[derive(Clone, Copy)]
#[repr(C)]
struct User {
    user_id: u8,
    username: Username,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("user_id", &self.user_id)
            .field("username", &unsafe {
                String::from_utf8_unchecked(self.username.to_vec())
            })
            .finish()
    }
}

impl User {
    pub const SIZE: usize = std::mem::size_of::<Self>();
    pub const DB_FILENAME: &'static str = "target/users.db\0";
    pub const LEN: usize = 10;
    fn new(user_id: u8) -> Self {
        assert!(Self::user_id_is_valid(user_id));
        let mut username = *b"user_00";
        username[5] = b'0' + (user_id / 10) % 10;
        username[6] = b'0' + user_id % 10;
        Self { user_id, username }
    }

    #[inline]
    const fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    /**
    ```no_run
    /// these can compile, Rust think *mut is superset of *const?
    fn as_mut_ptr(&mut self) -> *const Self {
        self as *mut Self
    }
    ```
    */
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }

    fn user_id_is_valid(user_id: u8) -> bool {
        user_id < Self::LEN as u8
    }

    /**
    ```text
    $ od -c target/user.db
    0000000 001   u   s   e   r   _   0   1 002   u   s   e   r   _   0   2
    0000020 003   u   s   e   r   _   0   3 004   u   s   e   r   _   0   4
    0000040 005   u   s   e   r   _   0   5 006   u   s   e   r   _   0   6
    0000060  \a   u   s   e   r   _   0   7  \b   u   s   e   r   _   0   8
    0000100  \t   u   s   e   r   _   0   9  \n   u   s   e   r   _   1   0
    0000120
    ```
    note that user_id=006 is escape to b'\a' in od
    */
    unsafe fn insert_sample_data() {
        let fp = libc::fopen(
            Self::DB_FILENAME.as_ptr().cast(),
            "w\0".as_ptr().cast(),
        );
        for user_id in 0..Self::LEN {
            let user = User::new(user_id as u8);
            libc::fwrite(user.as_ptr().cast(), Self::SIZE, 1, fp);
        }
        libc::fclose(fp);
    }

    unsafe fn select_all() -> Vec<User> {
        let fp = libc::fopen(
            Self::DB_FILENAME.as_ptr().cast(),
            "r\0".as_ptr().cast(),
        );
        let mut users = [std::mem::zeroed::<Self>(); Self::LEN];
        libc::fread(users.as_mut_ptr().cast(), Self::SIZE, Self::LEN, fp);
        libc::fclose(fp);
        users.to_vec()
    }

    /// since current doesn't support delete, so user_id=n must be record n
    unsafe fn find_user_by_id(user_id: u8) -> User {
        assert!(Self::user_id_is_valid(user_id));
        let fp = libc::fopen(
            Self::DB_FILENAME.as_ptr().cast(),
            "r\0".as_ptr().cast(),
        );
        let mut user = std::mem::zeroed::<User>();
        libc::fseek(fp, i64::from(user_id) * Self::SIZE as i64, libc::SEEK_SET);
        libc::fread(user.as_mut_ptr().cast(), Self::SIZE, 1, fp);
        libc::fclose(fp);
        user
    }

    /// since current doesn't support delete, so user_id=n must be record n
    unsafe fn update_username_by_id(user_id: u8, username: Username) {
        assert!(Self::user_id_is_valid(user_id));
        let offset = i64::from(user_id) * Self::SIZE as i64;
        let fp = libc::fopen(
            Self::DB_FILENAME.as_ptr().cast(),
            "r+\0".as_ptr().cast(),
        );
        let mut user = std::mem::zeroed::<User>();
        libc::fseek(fp, offset, libc::SEEK_SET);
        libc::fread(user.as_mut_ptr().cast(), Self::SIZE, 1, fp);
        user.username = username;
        libc::fseek(fp, offset, libc::SEEK_SET); // reset cursor after fread
        libc::fwrite(user.as_ptr().cast(), Self::SIZE, 1, fp);
        libc::fclose(fp);
    }

    /**
    ## O_RDONLY | O_WRONLY is Undefined Behaviour
    O_RDONLY, O_WRONLY, and O_RDWR do not specify individ‐ual bits.
    Rather, they define the low order two bits of flags,
    and are defined respectively as 0, 1, and 2.
    In other words, the combination O_RDONLY | O_WRONLY is a logical error
    */
    unsafe fn update_username_by_id_using_mmap(user_id: u8, username: Username) {
        assert!(Self::user_id_is_valid(user_id));
        let fd = libc::open(Self::DB_FILENAME.as_ptr().cast(), libc::O_RDWR);

        let mapped_addr = libc::mmap(
            0 as *mut libc::c_void,
            Self::LEN,
            libc::PROT_READ | libc::PROT_WRITE,
            // The segment changes are made in the file
            libc::MAP_SHARED,
            fd,
            0,
        );
        // mmap return 0 is ok, !0 is libc::MAP_FAILED
        if mapped_addr == libc::MAP_FAILED {
            libc::perror("\0".as_ptr().cast());
            libc::exit(1);
        }
        libc::close(fd); // mmap成功后就可以关闭fd，关闭fd不会影响mmap
        let users = mapped_addr as *mut [User; Self::LEN];
        (*users)[usize::from(user_id)].username = username;
        // sync mapping
        libc::msync(mapped_addr, Self::LEN * Self::SIZE, libc::MS_SYNC);
        // remove mapping
        libc::munmap(mapped_addr, Self::LEN * Self::SIZE);
    }
}

fn main() {
    unsafe {
        User::insert_sample_data();
        User::select_all();
        assert_eq!(User::find_user_by_id(3).username, *b"user_03");
        User::update_username_by_id(3, *b"tuesday");
        assert_eq!(User::find_user_by_id(3).username, *b"tuesday");
        User::update_username_by_id_using_mmap(3, *b"account");
        assert_eq!(User::find_user_by_id(3).username, *b"account");
    }
}
