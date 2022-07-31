//! 硬件异常处理模块

use core::panic;
use crate::println;
use super::context::Context; 

core::arch::global_asm!(include_str!("./interrupt.asm")); 

/// 初始化中断处理
/// 
/// 把中断入口 `__interrupt` 写入 `stvec` 中，并开启中断使能。
pub(super) fn init() {
    extern "C" {
        fn __interrupt(); 
    }
    let r = __interrupt as usize; 
    unsafe {
        core::arch::asm!("csrw stvec, {}", in(reg) r); 
    }
    println!("[[mod]] interrupt.handler has been initialized. "); 
}

/// 临时使用的中断处理入口
/// 
/// *interrupt.asm* 首先保存程序执行现场，而后将作为参数和 scause 以及 stval 一并传入此函数
/// scause, stval 在此处的改动不会影响到 s-trap 系 寄存器的值
/// context 内值的变化将会在函数执行结束后影响程式执行现场的内容 
#[no_mangle]
extern "C" fn handle_interrupt_backup(context: &mut Context, scause: usize, stval: usize) -> *mut Context { 
    use self::cause::*; 
    match scause.get_cause() {
        // 断点异常
        Cause::Exception(3) => {
            // println!("获取断点～"); 
            breakpoint(context)
        }

        // 无法访问 (load) 指定数据段地址 
        Cause::Exception(5) => {
            if stval == 0 {
                panic!("Null Pointer Exception!"); 
            } else {
                panic!("Fails to visit the address: 0x{:x}", stval); 
            }
        }

        #[cfg(not(feature = "time-disabled"))]
        // 时钟中断
        Cause::Interrupt(5) => {
            supervisor_timer(context)
        }

        // 其他情况
        _ => {
            panic!("Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}", scause.get_cause(), context, stval); 
            // unimplemented!("Still need to finish. "); 
        }
    }
}

/// 处理 ebreak 断点
/// 
/// 继续执行，其中 `sepc` 增加 2 字节，以跳过当前这条 `ebreak` 指令
fn breakpoint(context: &mut Context) -> *mut Context{
    println!("Breakpoint at 0x{:x}", context.sepc); 
    context.sepc += 2;
    context 
}

/// 处理时钟中断
/// 
/// 目前只会在 [`super::timer`] 模块中进行计数
#[cfg(not(feature = "time-disabled"))]
fn supervisor_timer(a: &mut Context) -> *mut Context {
    use crate::process::PROCESSOR;

    use super::*; 
    timer::tick();  
    PROCESSOR.lock().park_current_thread(a); 
    PROCESSOR.lock().prepare_next_thread()
}

mod cause {

    #[derive(Debug)]
    pub enum Cause {
        Interrupt(usize), 
        Exception(usize), 
    }

    pub trait GetCause {
        fn get_cause(self) -> Cause; 
    }

    impl GetCause for usize {
        fn get_cause(self) -> Cause {
            let v = self & 0x7FFF_FFFF_FFFF_FFFF; 
            if self & 0x8000_0000_0000_0000 != 0 {
                Cause::Interrupt(v) 
            } else {
                Cause::Exception(v) 
            }
        }
    }

}

// /// 处理缺页异常
// ///
// /// todo: 理论上这里需要判断访问类型，并与页表中的标志位进行比对
// fn page_fault(context: &mut Context, scause: usize, stval: usize) -> *mut Context {
//     static mut COUNT: usize = 0;
//     println!("page_fault {}", unsafe {
//         COUNT += 1;
//         COUNT
//     });
//     let current_thread = PROCESSOR.lock().current_thread();
//     let memory_set = &mut current_thread.process.inner().memory_set;
//     match memory_set.mapping.handle_page_fault(stval) {
//         Ok(_) => {
//             memory_set.activate();
//             context
//         }
//         Err(msg) => fault(msg, scause, stval),
//     }
// }

// / 出现未能解决的异常，终止当前线程
// fn fault(msg: &str, scause: usize, stval: usize) -> *mut Context {
//     println!(
//         "{:#x?} terminated: {}",
//         PROCESSOR.lock().current_thread(),
//         msg
//     );
//     use cause::GetCause; 
//     println!("cause: {:?}, stval: {:x}", scause.get_cause(), stval);

//     PROCESSOR.lock().kill_current_thread();
//     // 跳转到 PROCESSOR 调度的下一个线程
//     PROCESSOR.lock().prepare_next_thread()
// }
