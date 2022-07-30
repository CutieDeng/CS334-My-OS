//! 上下文支持

/// 可执行上下文内容
#[repr(C)]
#[derive(Debug)]
pub struct Context {
    pub x: [usize; 32], 
    pub sstatus: usize, 
    pub sepc: usize, 
}