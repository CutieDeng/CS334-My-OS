#![allow(unused)]

//! 调用 M 层的操作。

/// sbi 调用，设置为内联函数以减少执行动态开销。
#[inline(always)]
pub fn sbi_call(which: usize, arg0: usize, arg1:usize, arg2: usize) -> usize {
    let ret; 
    unsafe {
        core::arch::asm!("ecall", 
            in("a0") arg0, 
            in("a1") arg1, 
            in("a2") arg2, 
            in("a7") which, 
            lateout("a0") ret); 
    }
    ret 
}

const SBI_SET_TIMER: usize = 0; 
const SBI_CONSOLE_PUTCHAR: usize = 1; 
const SBI_CONSOLE_GETCHAR: usize = 2; 
const SBI_CLEAR_IPI: usize = 3; 
const SBI_SEND_IPI: usize = 4; 
const SBI_REMOTE_FENCE_I: usize = 5; 
const SBI_REMOTE_SFENCE_VMA: usize = 6; 
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7; 
const SBI_SHUTDOWN: usize = 8; 

/// 向控制台输出一个字符
/// 
/// 需要注意的是我们不能直接使用 char 类型用于输出。
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0); 
}

/// 从控制台中读取一个字符串。
/// 
/// 没有读取到字符串则返回 -1. 
pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

/// 关机，退出 QEMU. 
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0, ); 

    // 未定义行为代码，理应不被处理～
    // 为编译器补全该函数无返回值的定义实体
    // 在 sbi-call 完成后，程序应当退出，该代码不会被执行到。
    // Deprecated: 使用 unreachable 代替～
    // let r = unsafe {
    //     #[allow(invalid_value)]
    //     MaybeUninit::<!>::uninit().assume_init()
    // }; 
    // r 
    
    unsafe {
        core::hint::unreachable_unchecked() 
    }
}

/// 设置下一次时钟中断的时间
pub fn set_timer(time: usize) {
    sbi_call(SBI_SET_TIMER, time, 0, 0); 
}