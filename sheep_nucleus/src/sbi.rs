//! 调用 M 层的操作。
//! 
//! # sbi 是什么？
//! 
//! SBI 是 *Supervisor Binary Interface* 的 abbr. 
//! 
//! 在 RISC-V ISA 中，其 SBI 标准规定了类 Unix 操作系统之下的运行环境规范。
//! 该规范有多种实现，经典的实现为 **OpenSBI**, **RustSBI**. 
//! 
//! 在 RISC-V 架构中，存在着定义在 OS 之下的运行环境。该环境不仅将引导启动 RISC-V OS, 还将常驻后台，为操作系统提供一系列 binary interfaces. 
//! 操作系统通过其获取和操作硬件信息。
//! 
//! RISC-V 给出了此类环境和二进制接口的规范，称为「操作系统二进制接口」，即 SBI. 

#![allow(dead_code)]

/// sbi 调用
/// 
/// # 传入参数
/// 
/// - which: a7 寄存器的值
/// - arg0: a0 寄存器的值
/// - arg1: a1 寄存器的值
/// - arg2: a2 寄存器的值
/// 
/// # 返回值
/// 
/// 类型：[`usize`] 
/// 含义：SBI 接口调用的返回值
/// 
/// # 内联性
/// 
/// 设置为内联函数以减少执行动态开销
#[inline(always)]
pub unsafe fn sbi_call(which: usize, arg0: usize, arg1:usize, arg2: usize) -> usize {
    let ret; 
    {
        core::arch::asm!("ecall", 
            in("a0") arg0, 
            in("a1") arg1, 
            in("a2") arg2, 
            in("a7") which, 
            lateout("a0") ret); 
    }
    ret 
}

/// SBI 相关模数常量
/// 
/// 定义了 SBI 不同操作类型其对应的标志模数
pub mod magic_number {
    /// 预约时钟中断
    pub const SBI_SET_TIMER: usize = 0; 
    /// 控制台输出字符操作
    pub const SBI_CONSOLE_PUTCHAR: usize = 1; 
    /// 控制台获取字符操作
    pub const SBI_CONSOLE_GETCHAR: usize = 2; 
    /// 清空 IPI 信号
    pub const SBI_CLEAR_IPI: usize = 3; 
    /// 发送 IPI 信号
    pub const SBI_SEND_IPI: usize = 4; 
    pub const SBI_REMOTE_FENCE_I: usize = 5; 
    pub const SBI_REMOTE_SFENCE_VMA: usize = 6; 
    pub const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7; 
    /// 关机
    pub const SBI_SHUTDOWN: usize = 8; 
}

use magic_number::*; 

/// 向控制台输出一个字符
/// 
/// # 输入约束
/// 
/// 该 API 输入类型为 [`usize`], 但实际上只接受单字节输入，并以 **utf-8** 编码方案进行字符编码。
/// 对于 *unicode* 字符集下的非 ASCII 字符，其需要转化成多个字节，多次调用 [`console_putchar`] 才能输出该字符。
pub fn console_putchar(c: usize) {
    unsafe { sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0) } ; 
}

/// 从控制台中读取一个字符
/// 
/// # 输出描述
/// 
/// 该方法的输出类型为 [`usize`], 当读取不到字符串时（或读到 EOF 标记时）返回 [`usize::MAX`]. 
#[deprecated = "无法正确从命令行中获取输入"]
pub fn console_getchar() -> usize {
    unsafe {
        sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
    }
}

/// 关机
pub fn shutdown() -> ! {
    unsafe {
        sbi_call(SBI_SHUTDOWN, 0, 0, 0, ); 
        core::hint::unreachable_unchecked() 
    }
}

/// 设置下一次时钟中断的时间
/// 
/// # 输入参数
/// 
/// - time: 预约的中断发生时间戳
pub fn set_timer(time: usize) {
    unsafe { sbi_call(SBI_SET_TIMER, time, 0, 0) }; 
}