#![no_std]
#![feature(prelude_import, panic_info_message, never_type, alloc_error_handler, custom_test_frameworks)]

extern crate alloc; 

pub mod sbi; 
/// 提供常用的 sbi 函数——关机，输出。
pub use sbi::{shutdown, console_putchar} ; 

#[macro_use]
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

/// prelude 模块
/// 
/// # 自动导入
/// 
/// 该模块内的符号应当自动导入模块中
/// 
/// # prelude import 宏
/// 
/// todo: 不明作用，等待移除
#[prelude_import]
pub mod prelude {
    pub use crate::println;
}