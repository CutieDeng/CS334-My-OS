//! panic 库，实现 panic 和 abort 的基本功能。

use core::fmt::Write; 
use core::mem::MaybeUninit;

use crate::panic::u8array::U8Array;

use crate::println;
use crate::shutdown; 

mod u8array {
    use core::fmt; 

    pub struct U8Array<'a> (pub &'a mut [u8], usize ); 

    impl <'a> U8Array<'a> {
        /// 用一个 [`&mut [u8]`] 初始化 U8Array. 
        /// 
        /// 初始化后 U8Array 获得该切片的所有权。
        pub fn new(a: &'a mut [u8]) -> U8Array<'a> {
            Self (a, 0) 
        }
        pub fn clear(&mut self) {
            self.1 = 0; 
        }
        pub fn as_str(&self) -> &str {
            crate::println!("Invoke as str with size: {}", self.1); 
            core::str::from_utf8(&self.0[..self.1]).unwrap()
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

/// 宽度常量，描述了输出终端的宽度
const OUTPUT_WIDTH: usize = 140; 

/// 打印 panic 信息并 [`crate::shutdown`]. 
///
/// ### '#[panic_handler]' 属性
/// 声明此函数是 panic 的回调。
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // console::open_blue_print();
    let mut cached = [0u8; 1024]; 
    let mut cached = U8Array::new(&mut cached); 
    let mut tmp: [u8; 128] = unsafe { MaybeUninit::uninit().assume_init() }; 
    let mut tmp = U8Array::new(&mut tmp); 
    let _ = cached.write_str("\x1b[2J\x1b[1K"); 
    let mid_print = |w: &mut U8Array, s: &str| -> bool { 
        let len = s.chars().map(|a| if a.is_ascii() {1} else if a.is_control() {0} else {2}).sum::<usize>(); 
        // println!("Read '{}', len = {}. ", s, s.len()); 
        if s.is_empty() {
            let _ = write!(w, "\r\n"); 
            return true; 
        }
        if len > OUTPUT_WIDTH {
            return false 
        }
        let sp = (OUTPUT_WIDTH - len) / 2; 
        let _ = write!(w, "\x1b[{}C{}\r\n", sp, s); 
        return true 
    }; 
    let mid_print_lines = |w: &mut U8Array, s: &str, p: &dyn Fn(&mut U8Array, &str) -> bool| {
        let mut succeed = true;
        for s in s.lines() {
            succeed = p(&mut *w, s) && succeed;
        }
        return succeed; 
    }; 
    mid_print(&mut cached, ":("); 
    mid_print(&mut cached, "你的电脑遇到问题，需要关机。"); 
    mid_print(&mut cached, "我们只为您收集关键信息，然后为您关机。"); 
    mid_print(&mut cached, "如果您需要了解更多信息，请查看此错误。"); 
    if let Some(location) = info.location() {
        let _ = write!(tmp, "{}:{} {:?}", location.file(), location.line(), info.message().unwrap_or(&format_args!("无法解析的错误信息"))); 
    } else {
        let _ = write!(tmp, "{:?}", info.message().unwrap_or(&format_args!("无法解析的错误信息")));
    }
    mid_print_lines(&mut cached, tmp.as_str(), &mid_print); 
    if false 
    { tmp.clear(); } 
    println!("{}", cached.as_str()); 
    shutdown(); 
}

/// abort 函数
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()")
}