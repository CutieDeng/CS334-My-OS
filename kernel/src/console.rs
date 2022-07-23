//! The input & output support for console. 
//! 
//! # Format Output 
//! 
//! [`core::fmt::Write`] trait 
//! - [`write_str`] method 
//! - Other methods dependent with [`write_str`] 的 [`write_fmt`]. 
//! 
//! We should declare a type and realize the related [`write_str`] method, then we can easily use [`write_fmt`] for it. 
//! 
//! [`write_str`]: core::fmt::Write::write_str 
//! [`write_fmt`]: core::fmt::Write::write_fmt

use crate::sbi::*; 
use core::fmt::{self, Write}; 

/// 一个 [Zero-sized Type], 实现 [`core::fmt::Write`] trait 来进行格式化输出。
struct Stdout; 

impl Write for Stdout {

    /// The String output realization. 
    /// 
    /// ['console_putchar'] sbi accept a `usize` type input, but actually not a unicode support. 
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &b in s.as_bytes() {
            console_putchar(b.into()); 
        }
        Ok(())
    }

}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap(); 
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($args: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($args)+)?)); 
    } 
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?)) 
    } 
}