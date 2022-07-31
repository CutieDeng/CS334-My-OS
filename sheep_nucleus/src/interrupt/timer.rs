//! 预约和处理时钟中断
//! 
//! 

use crate::{println, sbi}; 
use riscv::register::time; 
use core::arch; 

/// 初始化时钟中断
/// 
/// 开启时钟中断使能，并且预约第一次时钟中断
pub(super) fn init() {
    // let p = || unsafe {
    //     let mut t: usize;
    //     use arch::asm; 
    //     asm!("csrr {}, sie", out(reg) t); 
    //     println!("sie: {:b}", t); 
    //     asm!("csrr {}, sstatus", out(reg) t); 
    //     println!("sstatus: {:b}", t); 
    // }; 
    // p(); 
    unsafe {
        // riscv 库的中断使能调度
        // sie::set_stimer(); 
        // sstatus::set_sie(); 
        use arch::asm; 
        // 内联汇编实现
        // asm!("csrr {p}, sie", "ori {p}, {p}, 0x20", "csrw sie, {p}", p = out(reg) _); 
        asm!("csrr {s}, sstatus", "ori {s}, {s}, 0x2", "csrw sstatus, {s}", s = out(reg) _); 
    }
    // p(); 
    set_next_timeout(); 
    println!("[[mod]] interrupt.timer has been initialized. ")
}

/// 时钟中断的间隔，单位是 CPU 时钟周期
const INTERVAL: usize = 10_000_000; 

/// 设置下一次时钟中断
/// 
/// 获取当前的时间，加上中断间隔，通过 SBI 调用预约下一次中断
fn set_next_timeout() {
    sbi::set_timer(time::read() + INTERVAL); 
}

use core::sync::atomic; 

/// 触发时钟中断计数
pub static TICKS: atomic::AtomicUsize = atomic::AtomicUsize::new(0); 

/// 每一次时钟中断时调用
/// 
/// 设置下一次时钟中断，同时令 TICKS 的计数 +1. 
/// 当 TICKS 计数达到 100 时，进行调度
pub fn tick() {
    set_next_timeout(); 
    use atomic::Ordering; 
    let t = TICKS.fetch_add(1, Ordering::Relaxed) + 1; 
    // if t % 100 == 0 {
        crate::println!("{} ticks", t); 
    // }
}