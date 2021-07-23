//! 如何通过getopt系统调用实现任意顺序的参数都能正常解析
//! optstr的语法可读性太差了，也不是特点重要的知识，项目中也用不上(clap写命令行应用不香么)

extern "C" {
    static optarg: *const libc::c_char;
    static optopt: libc::c_int;
    static optind: libc::c_int;
}

/**
output:
```text
option: i
option: l
option: r
filename: "fred.c"
unknown option: q
argument: "hi there"
```
*/
#[test]
#[allow(clippy::similar_names)]
fn main() {
    let argv = vec![
        "argv[0]",
        "-i\0",
        "-lr\0",
        "hi there\0",
        "-f\0",
        "fred.c\0",
        "-q\0",
    ]
    .into_iter()
    .map(|arg| arg.as_ptr().cast())
    .collect::<Vec<*const libc::c_char>>();
    let argc = argv.len() as i32;
    let optstr = ":if:lr\0";
    unsafe {
        loop {
            // alternative is getopt_long, but need one more arg longopts: Vec<option>
            let opt = libc::getopt(argc, argv.as_ptr().cast(), optstr.as_ptr().cast());
            if opt == -1 {
                break;
            }
            match opt as u8 {
                b'i' | b'l' | b'r' => println!("option: {}", opt as u8 as char),
                b'f' => {
                    libc::printf("filename: %s\n\0".as_ptr().cast(), optarg);
                }
                b':' => println!("option needs a value"),
                b'?' => println!("unknown option: {}", optopt as u8 as char),
                _ => unreachable!(),
            }
        }
        // optind indicates where the remaining arguments unused
        // getopt would rewrite argv's order, move unused to the end of array
        for i in optind..argc {
            libc::printf("argument: %s\n\0".as_ptr().cast(), argv[i as usize]);
        }
    }
}
