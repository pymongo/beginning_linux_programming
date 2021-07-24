use crate::{gethostbyname, inet_ntoa, syscall, SOCKADDR_IN_LEN};

#[allow(clippy::ptr_as_ptr, clippy::cast_ptr_alignment)]
fn nslookup(hostname: &str) -> libc::sockaddr_in {
    let hostname = std::ffi::CString::new(hostname).unwrap();
    let hostname = hostname.as_ptr().cast();
    let hostent = unsafe { gethostbyname(hostname) };
    if hostent.is_null() {
        panic!("Invalid hostname or DNS lookup lookup failed");
    }
    let hostent = unsafe { *hostent };
    let remote_addr = unsafe { *(*hostent.h_addr_list as *mut libc::in_addr) };
    syscall!(printf(
        "PING %s (%s) 64 bytes of data\n\0".as_ptr().cast(),
        hostname,
        inet_ntoa(remote_addr),
    ));
    libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: 0,
        sin_addr: remote_addr,
        sin_zero: unsafe { std::mem::zeroed() },
    }
}

/// icmphdr.type usually use ICMP_ECHO
const ICMP_ECHO: u8 = 8;

#[derive(Clone)]
#[repr(C)]
struct icmphdr {
    type_: u8,
    code: u8,
    checksum: u16,
    un: un,
}

#[derive(Clone, Copy)]
#[repr(C)]
union un {
    echo: echo,
    gateway: u32,
    frag: frag,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct echo {
    id: u16,
    sequence: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct frag {
    __glibc_reserved: u16,
    mtu: u16,
}

#[derive(Clone)]
#[repr(C)]
struct Packet {
    hdr: icmphdr,
    msg: [u8; PACKET_LEN - std::mem::size_of::<icmphdr>()],
}

const PACKET_LEN: usize = 64;

#[test]
fn main() {
    let remote_addr = nslookup("baidu.com");

    // SOCK_RAW need sudo/root permission
    // let sockfd = libc::socket(libc::AF_INET, libc::SOCK_RAW, (*libc::getprotobyname("ICMP\0".as_ptr().cast())).p_proto);
    let sockfd = syscall!(socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_ICMP));
    syscall!(fcntl(sockfd, libc::F_SETFL, libc::O_NONBLOCK));
    syscall!(setsockopt(
        sockfd,
        libc::SOL_IP,
        libc::IP_TTL,
        (&64 as *const i32).cast(),
        std::mem::size_of::<i32>() as u32
    ));

    for _ in 0..10 {
        let mut packet: Packet = unsafe { std::mem::zeroed() };
        let mut addr = remote_addr;
        let mut addrlen = SOCKADDR_IN_LEN;
        println!("before recv");
        let recvfrom_ret = unsafe {
            libc::recvfrom(
                sockfd,
                (&mut packet as *mut Packet).cast(),
                PACKET_LEN,
                0,
                (&mut addr as *mut libc::sockaddr_in).cast(),
                &mut addrlen,
            )
        };
        if recvfrom_ret > 0 {
            println!("ping success");
            std::process::exit(libc::EXIT_SUCCESS);
        }
        println!("after recv");

        packet = unsafe { std::mem::zeroed() };
        packet.hdr.type_ = ICMP_ECHO;
        for i in 0..packet.msg.len() - 1 {
            packet.msg[i] = i as u8 + b'0';
        }
        println!("{:?}", packet.msg);
        // 先写死，在 linux_command_rewritten_in_rust 的 repo 去实现 pnet::util::checksum 算法
        packet.hdr.checksum = 3772;
        // packet.hdr.checksum = pnet::util::checksum(&packet.msg, 0);

        syscall!(sendto(
            sockfd,
            (&packet as *const Packet).cast(),
            PACKET_LEN,
            0,
            (&remote_addr as *const libc::sockaddr_in).cast::<libc::sockaddr>(),
            SOCKADDR_IN_LEN,
        ));

        syscall!(usleep(300 * 1000));
    }
    eprintln!("ping failed!");
}
