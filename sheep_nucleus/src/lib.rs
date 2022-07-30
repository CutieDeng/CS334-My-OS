#![no_std]
#![feature(panic_info_message, never_type, alloc_error_handler)]

extern crate alloc; 

pub mod sbi; 
/// 提供常用的 sbi 函数——关机，输出。
pub use sbi::{shutdown, console_putchar} ; 

pub mod console; 

/// panic 库实现了操作系统运行过程中发生不可处理错误时的情形
/// 
/// # private 
/// 
/// panic 库不应该暴露给使用者使其了解实现细节。
mod panic; 

pub use memory::FrameTracker; 

pub mod interrupt;
pub mod memory; 

#[cfg(feature = "cutie-io-backup")]
pub mod debug_helper; 

pub mod prelude {
    pub use crate::println;
}