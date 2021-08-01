/**
```text
| inet_aton("127.0.0.1") |
+------------------------+
|             2130706433 |
```
*/
#[test]
fn get_host_byte_order() {
    // env CARGO_CFG_TARGET_ENDIAN in build.rs
    union IPv4 {
        int: u32,
        bytes: [u8; 4],
    }
    let ip = IPv4 { int: 1 };
    // 大端序符合人类阅读习惯，最低位在最右
    // network byte order == bigger endian
    if unsafe { ip.bytes } == [0, 0, 0, 1] {
        println!("bigger endian == MSB");
    }
    // 现代 CPU 主流用小端序，内存地址较低的数组第 0 项是最低项，最低位在最左
    // 注意 Java 虚拟机默认用 bigger-endian
    if unsafe { ip.bytes } == [1, 0, 0, 0] {
        println!("little endian == LSB");
    }
}
