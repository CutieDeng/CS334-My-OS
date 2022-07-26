#![no_std]
#![no_main]
#![feature(panic_info_message, never_type, alloc_error_handler)]

extern crate alloc; 
extern crate log;

use alloc::vec::Vec;
use log::LevelFilter;

core::arch::global_asm!(include_str!("entry.asm")); 

use sheep_nucleus::{*, memory::PAGE_SIZE}; 

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
    // println!("{}\n\n", 21); 
    unsafe { ebreak() }; 
    {
        let a: usize; 
        unsafe {
            core::arch::asm!("csrr {}, satp", out(reg) a); 
        }
        print!("Satp value: 0x"); 
        output_val_0x(a); 
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
    {
        extern "C" {
            fn boot_page_table(); 
            fn data_start(); 
            fn kernel_end(); 
            fn boot_stack(); 
            fn boot_stack_top(); 
        }
        output_val_with_hint("boot page table addr: 0x", {boot_page_table as usize}); 
        output_val_with_hint("data start: 0x", {data_start as usize}); 
        let boot_page_table_addr: usize; 
        unsafe { core::arch::asm!("lui {}, %hi(boot_page_table)", out(reg) boot_page_table_addr);  }
        output_val_with_hint("Boot page table addr: 0x", boot_page_table_addr);  
        // println!("boot page table addr: 0x{:x}", { boot_page_table as usize } ); 
        // println!("data start: 0x{:x}", { data_start as usize } ); 
        // println!("kernel end: 0x{:x}", { kernel_end as usize } ); 
        // println!("boot stack: 0x{:x}", { boot_stack as usize } ); 
        // println!("boot stack top: 0x{:x}", { boot_stack_top as usize } ); 
        // Fetch the value of sp: 
        let sp: usize; 
        unsafe { core::arch::asm!("mv {0}, sp", out(reg) sp, ); } 
        // println!("The value of the stack pointer is: 0x{:x}", sp); 
        output_val_with_hint("The value of the stack pointer is: 0x", sp); 
        // output_pte(boot_page_table as usize); 
    }
    println!("你好，我的 rCore. "); 
    {
        println!("{}, {}", "hello", "world"); 
        println!("{}, {}", "hello", "world"); 
        println!("{}, {}", "hello", "world"); 
        let t = 18; 
        // println!("My age is {}", t); 
        #[allow(unconditional_panic)]
        let t = 3 / 0; 
    }
    println!("关机！"); 
    shutdown();
}