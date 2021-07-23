use beginning_linux_programming::{gethostbyname, inet_ntoa, SOCKADDR_IN_LEN};

unsafe fn nslookup(hostname: &str) -> libc::sockaddr_in {
    let hostname = std::ffi::CString::new(hostname).unwrap();
    let hostname = hostname.as_ptr().cast();
    let hostent = gethostbyname(hostname);
    if hostent.is_null() {
        panic!("Invalid hostname or DNS lookup lookup failed");
    }
    let hostent = *hostent;
    let remote_addr = *(*hostent.h_addr_list as *mut libc::in_addr);
    // PING baidu.com (39.156.69.79) 56(84) bytes of data.
    libc::printf(
        "PING %s (%s) 64 bytes of data\n\0".as_ptr().cast(),
        hostname,
        inet_ntoa(remote_addr),
    );
    libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: 0,
        sin_addr: remote_addr,
        sin_zero: std::mem::zeroed(),
    }
}

#[repr(C)]
struct icmphdr {
    type_: u8,
    code: u8,
    checksum: u16,
    _union_padding: u32,
}

const ICMP_ECHO: u8 = 8;

fn main() {
    unsafe {
        let remote_addr = nslookup("baidu.com");

        let sockfd = libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_ICMP);
        if sockfd == -1 {
            panic!("SOCK_RAW need **sudo** permission");
        }

        let ping_package = icmphdr {
            type_: ICMP_ECHO,
            ..std::mem::zeroed()
        };

        let a = libc::sendto(
            sockfd,
            (&ping_package as *const icmphdr).cast(),
            std::mem::size_of::<icmphdr>(),
            0,
            (&remote_addr as *const libc::sockaddr_in).cast::<libc::sockaddr>(),
            SOCKADDR_IN_LEN,
        );
        dbg!(a);
        if a == -1 {
            libc::perror(std::ptr::null());
        }
    }
}
