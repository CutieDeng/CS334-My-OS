#![no_std]
#![no_main]
#![feature(panic_info_message, never_type, alloc_error_handler)]

extern crate alloc; 

use alloc::boxed::Box;
use alloc::vec::Vec; 

core::arch::global_asm!(include_str!("entry.asm")); 

mod sbi;
mod interrupt;
mod memory;
mod console;
mod panic;
// mod log;

unsafe fn ebreak() {
    use core::arch::asm; 
    asm!("ebreak"); 
}
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt::init(); 
    memory::init();
    // log::warn!("This is an example message.");
    eprintln!("打印红色信息测试!");
    {
        println!("栈地址初始位置为：0x{:x}. ", 
            &0 as *const i32 as usize); 
    }

    println!("你好，我的 rCore. "); 

    println!("内核结束地址：0x{:x}", memory::get_kernel_end()); 

    // let mut t = Vec::new();
    // for i in 0..20_000_000 {
    //     t.push(0);
    //     if i % 10000 == 0 {
    //         let p = (&t[i]) as *const i32 as usize;
    //         println!("t[{}]'s address is {:x}", i, p);
    //     }
    // }
    println!("关机！"); 
    sbi::shutdown();
}