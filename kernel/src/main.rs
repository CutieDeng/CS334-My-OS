#![no_std]
#![no_main]
#![feature(panic_info_message, never_type, alloc_error_handler)]

extern crate alloc; 

use alloc::boxed::Box;
use alloc::vec::Vec; 

core::arch::global_asm!(include_str!("entry.asm")); 

use kernel::*; 

unsafe fn ebreak() {
    use core::arch::asm; 
    asm!("ebreak"); 
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt::init(); 
    memory::init(); 

    {
        println!("栈地址初始位置为：0x{:x}. ", 
            &0 as *const i32 as usize); 
    }

    println!("你好，我的 rCore. "); 
    // for _ in 0..15_000_000 { }

    let v = Box::new(21); 
    assert_eq!(*v, 21); 
    core::mem::drop(v); 

    let mut vec = Vec::new(); 
    for i in 0..10000 {
        vec.push(i); 
    }
    assert_eq! (vec.len(), 10000); 
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(i, value); 
    }

    // unsafe {
    //     ebreak(); 
    //     core::arch::asm!("li x0, 0", 
    //         "jr x0"); 
    // }

    println!("内核结束地址：0x{:x}", memory::get_kernel_end()); 

    for _ in 0..20_000_000 {} 

    for _ in 0..2 {
        let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }

    println!("关机！"); 
    sbi::shutdown(); 
}