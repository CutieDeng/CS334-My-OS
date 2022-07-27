#![no_std]
#![no_main]
#![feature(panic_info_message, never_type, alloc_error_handler)]

extern crate alloc; 
extern crate log;

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
    {
        let mut a: usize; 
        unsafe {
            core::arch::asm!("csrr {}, satp", out(reg) a); 
        }
        println!("satp: 0x{:x}", a); 
        unsafe {
            core::arch::asm!("mv {}, sp", out(reg) a); 
        }
        println!("sp: 0x{:x}", a); 
        extern "C" {
            fn kernel_end(); 
        }
        println!("END of kernel: 0x{:x}", kernel_end as usize); 
    }
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
    println!("你好，我的 rCore. "); 
    {
        for i in 0..100000000 {
            use alloc::boxed::Box; 
            let t = Box::new(3); 
            if i % 100000 == 0 {
                println!("The address is {:p}", t.as_ref()); 
            }
            core::mem::forget(t); 
        }
    }
    println!("关机！"); 
    shutdown();
}