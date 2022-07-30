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
    if false 
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

    println!("test"); 

    sheep_logger::init().expect("日志管理器加载失败！");
    sheep_logger::set_level(LevelFilter::Info);
    
    log::info!("你好，我的 rCore. "); 
    {
        for i in 0..100000000 {
            use alloc::boxed::Box; 
            let t = Box::new(3); 
            if i % 100000 == 0 {
                log::warn!("The address is {:p}", t.as_ref()); 
            }
            core::mem::forget(t); 
        }
    }
    log::warn!("关机！"); 
    shutdown();
}

trait Run {
    fn run(); 
}