#![no_std]
#![no_main]
#![feature(panic_info_message, never_type, alloc_error_handler)]

extern crate alloc; 
extern crate log;

use alloc::vec::Vec;
use log::LevelFilter;

core::arch::global_asm!(include_str!("entry.asm")); 

use sheep_nucleus::*; 

mod sheep_logger;

#[inline(always)] 
#[allow(dead_code)]
unsafe fn ebreak() {
    use core::arch::asm; 
    asm!("ebreak"); 
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt::init(); 
    memory::init();
    sheep_logger::init().expect("日志管理器加载失败！");
    sheep_logger::set_level(LevelFilter::Trace);
    if cfg!(feature = "twocat-log-debug-itself") {
        log::error!("This is an error message.");
        log::warn!("This is an warning message.");
        log::info!("This is an info message.");
        log::debug!("This is an warning message.");
        log::trace!("This is an trace message.");
        // .. 
        eprintln!("打印红色信息测试!");
        for c in 'A'..='Z'{
            eprint!("{}", c);
        }
        eprintln!();
    }
    {
        // Cheeting codes! No the actually the address of the stack! 
        // println!("栈地址初始位置为：0x{:x}. ", 
        //     &0 as *const i32 as usize); 
    }
    {
        extern "C" {
            fn boot_page_table(); 
            fn data_start(); 
            fn kernel_end(); 
            fn boot_stack(); 
            fn boot_stack_top(); 
        }
        println!("boot page table addr: 0x{:x}", { boot_page_table as usize } ); 
        println!("data start: 0x{:x}", { data_start as usize } ); 
        println!("kernel end: 0x{:x}", { kernel_end as usize } ); 
        println!("boot stack: 0x{:x}", { boot_stack as usize } ); 
        println!("boot stack top: 0x{:x}", { boot_stack_top as usize } ); 
        // Fetch the value of sp: 
        let sp: usize; 
        unsafe { core::arch::asm!("mv {0}, sp", out(reg) sp, ); } 
        println!("The value of the stack pointer is: 0x{:x}", sp); 
    }
    println!("你好，我的 rCore. "); 
    println!("关机！"); 
    shutdown();
}