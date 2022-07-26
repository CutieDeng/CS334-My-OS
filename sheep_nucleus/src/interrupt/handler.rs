use core::panic;

use crate::println;

use super::context::Context; 

core::arch::global_asm!(include_str!("./interrupt.asm")); 

/// 初始化中断处理
/// 
/// 把中断入口 [`__interrupt`] 写入 `stvec` 中，并开启中断使能。
pub fn init() {
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
#[allow(dead_code, unused_variables)]
#[no_mangle]
extern "C" fn handle_interrupt_backup(context: &mut Context, scause: usize, stval: usize) { 
    use self::cause::*; 
    match scause.get_cause() {
        // 断点异常
        Cause::Exception(3) => {
            // println!("获取断点～"); 
            crate::debug::output_val_with_hint("[Handle] Breakpoint context addr: ", context as *mut Context as usize); 
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

        // 时钟中断
        Cause::Interrupt(5) => {
            #[cfg(feature = "time-enabled")]
            supervisor_timer(context); 
        }

        // 其他情况
        _ => {
            // panic!("Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}", scause.cause(), context, stval); 
            unimplemented!("Still need to finish. "); 
        }
    }
}

/// 处理 ebreak 断点
/// 
/// 继续执行，其中 `sepc` 增加 2 字节，以跳过当前这条 `ebreak` 指令
fn breakpoint(context: &mut Context) {
    crate::debug::output_val_with_hint("[Handle In] breakpoint context addr: ", context as *mut Context as usize); 
    println!("Breakpoint at 0x{:x}", context.sepc); 
    crate::debug::output_val_with_hint("[After println] Epc: ", context.sepc); 
    crate::debug::output_val_with_hint("[After println] Epc: ", context.sepc); 
    crate::debug::output_val_with_hint("[After println] Epc: ", context.sepc); 
    // 该语句无法正确的设置 context 的 sepc 的值变化过程！
    context.sepc += 2;
}

/// 处理时钟中断
/// 
/// 目前只会在 [`timer`] 模块中进行计数
#[cfg(feature = "time-enabled")]
fn supervisor_timer(_: &Context) {
    use super::*; 
    timer::tick();  
}

mod cause {

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