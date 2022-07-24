use super::address::*; 

/// 操作系统动态分配内存所用的堆大小 (8M) 
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000; 

use lazy_static::lazy_static; 

lazy_static! {
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as usize);
}

pub fn get_kernel_end() -> usize {
    return kernel_end as usize; 
}

extern "C" {
    fn kernel_end(); 
}

/// 页、帧大小，必须是 2^n 
pub const PAGE_SIZE: usize = 4096; 

/// 可以访问的内存区域起始地址
pub const MEMORY_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x8000_0000); 

/// 可以访问的内存区域结束地址
pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000); 
