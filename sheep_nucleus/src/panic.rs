//! panic 库，实现 panic 和 abort 的基本功能。

use core::fmt::{Write, Arguments};

use crate::panic::u8array::U8Array;
/// 打印 panic 信息并 [`crate::shutdown`]. 
///
/// ### '#[panic_handler]' 属性
/// 声明此函数是 panic 的回调。

use crate::println;
use crate::shutdown; 

mod u8array {
    use core::fmt; 

    pub struct U8Array<'a> (pub &'a mut [u8], usize ); 

    impl <'a> U8Array<'a> {
        pub fn new(a: &'a mut [u8]) -> U8Array<'a> {
            Self (a, 0) 
        }
    }

    impl <'a> core::convert::Into<&'a str> for U8Array<'a> {
        fn into(self) -> &'a str {
            core::str::from_utf8(&self.0[..self.1]).unwrap_or("!")
        }
    }

    impl <'a> core::fmt::Write for U8Array<'a> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            for s in s.as_bytes() {
                if self.1 >= self.0.len() {
                    return Err(fmt::Error)
                }
                self.0[self.1] = *s; 
                self.1 += 1; 
            }
            Ok(())
        }
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // console::open_blue_print();
    let mut cached = [0u8; 1024]; 
    let mut cached = U8Array::new(&mut cached); 
    for _ in 0..60 {
        let _ = cached.write_str("\x1bD"); 
    }
    let _ = cached.write_str("\x1b[2J\x1b[1K"); 
    let mid_print = |w: &mut U8Array, s: &str| {
        for s in s.lines() {
            let len = s.chars().map(|a| if a.is_ascii() {1} else if a.is_control() {0} else {2}).sum::<usize>(); 
            if s.is_empty() {
                let _ = write!(*w, "\r\n"); 
                continue; 
            }
            if len > 40 {
                continue; 
            }
            let sp = (80 - len) / 2; 
            let _ = write!(*w, "\x1b[{}C{}\r\n", sp, s); 
        }
    }; 
    mid_print(&mut cached, 
        ":(\r\n你的电脑遇到问题，需要关机。\n我们只为您收集关键信息，然后为您关机。\n\n\n如果您需要了解更多信息，请查看此错误。\n"); 
    // if let Some(location) = info.location() {
    //     mid_print(&mut cached, &format_args!("{}:{} '{:?}'", location.file(), location.line(), {
    //         info.message().unwrap_or(&format_args!("【无法解析错误信息】"))
    //     })); 
    // } else {
    //     mid_print(&mut cached, &format_args!("{:?}", info.message()))
    // }
    let cached: &str = cached.into(); 
    println!("{}", cached); 
    println!("size of c: {}", cached.len()); 
    shutdown(); 
}

/// abort 函数
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()")
}