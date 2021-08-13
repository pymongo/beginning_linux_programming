#![feature(unboxed_closures)]
#![feature(fn_traits)]

struct F;

impl FnOnce<(&str,)> for F {
    type Output = String;
    extern "rust-call" fn call_once(self, args: (&str,)) -> String {
        format!("arg is ({},)", args.0)
    }
}

impl FnOnce<(&str, u64)> for F {
    type Output = String;
    extern "rust-call" fn call_once(self, args: (&str, u64)) -> String {
        format!("arg is ({}, {})", args.0, args.1)
    }
}

fn main() {
    dbg!(F("mike"));
    dbg!(F("mike", 1u64));
}
