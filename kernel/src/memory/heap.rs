use crate::println;

use super::config::*; 

use buddy_system_allocator::{LockedHeap}; 

/// 进行动态内存分配所用的堆空间
/// 
/// 大小为 [`KERNEL_HEAP_SIZE`] 
/// 这段空间编译后会被放在操作系统执行程序的 bss 段
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE]; 

/// 堆，动态内存分配器
/// 
/// ### `#[global_allocator]` 
/// [`LockedHeap`] 实现了 [`alloc::alloc::GlobalAlloc`] trait. 
/// 可以为全局需要用到堆的地方分配空间。例如 `Box` `Arc` 等
#[global_allocator]
static HEAP: LockedHeap<{KERNEL_HEAP_SIZE}> = LockedHeap::empty(); 

/// 初始化操作系统运行时堆空间
pub(super) fn init() {
    // 告诉分配器使用这一段预留的空间作为堆
    unsafe {
        HEAP.lock().init(
            HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE
        )
    }
    println!("[[mod]] memory.heap has been initialized. "); 
}
