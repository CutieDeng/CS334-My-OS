#[repr(C)]
#[derive(Debug)]
pub struct Context {
    pub x: [usize; 32], 
    pub sstatus: usize, 
    pub sepc: usize, 
}