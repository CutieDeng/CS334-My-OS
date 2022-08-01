#![no_std]
#![no_main]
#![feature(panic_info_message, never_type, alloc_error_handler, )]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)] 
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests. ", tests.len()); 
    for test in tests {
        test(); 
    }
}

extern crate alloc; 
extern crate log;

use core::arch::asm;

use log::LevelFilter;

// #[macro_use(print, println)]
// extern crate sheep_nucleus as sn; 

// use sn::*; 
use sheep_nucleus::{*, process::{PROCESSOR, Process, Thread}}; 
use alloc::sync::Arc;

core::arch::global_asm!(include_str!("entry.asm")); 

mod sheep_logger;

#[inline(always)] 
#[allow(dead_code)]
unsafe fn ebreak() {
    asm!("ebreak"); 
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt::init(); 
    memory::init();
    process::init(); 

    let remap = memory::mapping::MemorySet::new_kernel().unwrap(); 
    remap.activate(); 

    sheep_logger::init().expect("日志管理器加载失败！");
    sheep_logger::set_level(LevelFilter::Trace);

    log::info!("kernel remap! "); 

    {
        let t: usize; 
        unsafe {
            asm!("csrr {}, sscratch", out(reg) t); 
        }
        log::trace!("Before ebreak, the value of kernel stack top is 0x{:x}", t); 
    }
    unsafe { ebreak(); } 
    {
        let t: usize; 
        unsafe {
            asm!("csrr {}, sscratch", out(reg) t); 
        }
        log::trace!("After ebreak, the value of kernel stack top is 0x{:x}", t); 
    }
    
    if true {
        // 内核重映射测试
        // 测时 rodata 无法被 write. 
        let ot = "test"; 
        let t = ot.as_ptr() as *mut u8;
        unsafe {
            // *t = b'a'; 
            t.write(b'a'); 
        } 
        log::info!("对可读字符串 test 进行修改， exec \"test\"[0] = 'a'. "); 
        log::info!("运行结果：test = {}", ot); 
        log::debug!("运行涉及到的 PTE 信息：{:?}", 
            {
                let va: memory::VirtualAddress = t.into(); 
                let vpn = memory::VirtualPageNumber::floor(va); 
                memory::mapping::Mapping::crate_pte(vpn)
            }.unwrap()); 
        println!(); 
    }

    if true {
        use alloc::string::String;  
        log::info!("通过 alloc::String 在堆上获取一个 String, 其值为 test. "); 
        log::info!("将其转化为一个 &str type, 并同样地修改其第一个元素的值为 'a'. "); 
        let ot = String::from("test"); 
        let ot = ot.as_str(); 
        // 在 writtable 页进行同样的操作
        let t = ot.as_ptr() as *mut u8;
        unsafe {
            // *t = b'a'; 
            t.write(b'a'); 
        } 
        log::info!("修改结果：test = {}", ot); 
        log::debug!("运行涉及的 PTE 信息： {:?}", {
            let va: memory::VirtualAddress = t.into(); 
            let vpn = memory::VirtualPageNumber::floor(va); 
            memory::mapping::Mapping::crate_pte(vpn)
        }.unwrap()); 
        println!(); 
    }

    if true {
        extern "C" {
            fn rust_main (); 
        }
        log::info!("内核的 rust 运行代码执行入口：rust_main 函数位置 0x{:x}", rust_main as usize);
        log::info!("其位置对应的 PTE 信息：{:?}", {
            let va: memory::VirtualAddress = (rust_main as usize).into(); 
            let vpn = memory::VirtualPageNumber::floor(va); 
            memory::mapping::Mapping::crate_pte(vpn)
        }.unwrap()); 
        println!(); 
    }

    #[cfg(test)]
    test_main(); 

    log::info!("你好，我的 rCore. "); 

    {
        // use sheep_nucleus::process::PROCESSOR; 
        let mut processor = PROCESSOR.lock(); 
        log::info!("获得 cpu 抽象");
        let kernel_process = Process::new_kernel(); 
        let kernel_process = kernel_process.unwrap(); 
        log::info!("构建进程成功"); 
        // for i in 1..9usize {
        processor.add_thread(create_kernel_thread(kernel_process.clone(), sample_process as usize, Some(&[0]))); 
        log::info!("创建线程中"); 
        // }
    }

    {
        let t: usize; 
        unsafe {
            asm!("mv {}, sp", out(reg) t); 
        }
        log::trace!("Before ebreak, the value of user stack top is 0x{:x}", t); 
    }
    unsafe { ebreak(); } 
    {
        let t: usize; 
        unsafe {
            asm!("mv {}, sp", out(reg) t); 
        }
        log::trace!("After ebreak, the value of user stack top is 0x{:x}", t); 
    }

    shutdown(); 

    {
        extern "C" {
            fn __restore (context: usize); 
        }
        let context = PROCESSOR.lock().prepare_next_thread(); 
        println!("即将 j __restore. "); 
        unsafe {
            __restore(context as usize); 
        }
    }

    unreachable!(); 
}

trait Run {
    fn run(); 
}

fn kernel_thread_exit() {
    PROCESSOR.lock().current_thread().as_ref().inner().dead = true; 
    unsafe {
        asm!("ebreak"); 
    }
}

pub fn create_kernel_thread (
    process: Arc<Process>, 
    entry_point: usize, 
    arguments: Option<&[usize]>, 
) -> Arc<Thread> {
    // 创建进程
    let thread = Thread::new(process, entry_point, arguments).unwrap(); 
    // 设置进程的返回地址为 kernel thread exit. 
    thread.as_ref().inner().context.as_mut().unwrap().set_ra(kernel_thread_exit as usize); 
    thread
}

fn sample_process (message: usize) {
    println!("Hello from kernel thread: {}", message ); 
}