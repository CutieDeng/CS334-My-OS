//! 硬件异常处理模块

use core::panic;
use crate::println;
use super::context; 

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
extern "C" fn handle_interrupt_backup(context: &mut context::Context, scause: usize, stval: usize) { 
    use self::cause::*; 
    match scause.get_cause() {
        // 断点异常
        Cause::Exception(3) => {
            // println!("获取断点～"); 
            breakpoint(context); 
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
            supervisor_timer(context); 
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
fn breakpoint(context: &mut context::Context) {
    println!("Breakpoint at 0x{:x}", context.sepc); 
    context.sepc += 2;
}

/// 处理时钟中断
/// 
/// 目前只会在 [`super::timer`] 模块中进行计数
#[cfg(not(feature = "time-disabled"))]
fn supervisor_timer(_: &context::Context) {
    use super::*; 
    timer::tick();  
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