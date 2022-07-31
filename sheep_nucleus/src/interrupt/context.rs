//! 上下文支持

/// 可执行上下文内容
#[repr(C)]
#[derive(Debug)]
pub struct Context {
    pub x: [usize; 32], 
    pub sstatus: usize, 
    pub sepc: usize, 
}

impl Context {
    pub fn new (sp: usize, pc: usize, args: Option<&[usize]>, is_user: bool) -> Self {
        unimplemented!()
    }
}