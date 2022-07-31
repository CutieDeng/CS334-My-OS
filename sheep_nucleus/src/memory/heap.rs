//! 内核堆管理模块
//! 
//! 该模块定义了内核态的堆内存空间 (大小为 8M) 相关信息。
use super::*; 

use buddy_system_allocator::LockedHeap; 

/// 进行动态内存分配所用的堆空间
/// 
/// 大小为 [`KERNEL_HEAP_SIZE`] 
/// 这段空间编译后会被放在操作系统执行程序的 bss 段
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE]; 

/// 堆，动态内存分配器
/// 
/// ### `#[global_allocator]` 
/// 
/// [`LockedHeap`] 实现了 [`alloc::alloc::GlobalAlloc`] trait. 
/// 可以为全局需要用到堆的地方分配空间。例如 `Box` `Arc` 等
#[global_allocator]
static HEAP: LockedHeap<31> = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("蛮羊系统堆内存分配异常。")
}

/// 初始化操作系统运行时堆空间
pub(super) fn init() {
    // 告诉分配器使用这一段预留的空间作为堆
    unsafe {
        HEAP.lock().init(
            HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE
        )
    }
    #[cfg(feature = "cutie-log-memory")]
    {
        println!("[[mod]] memory.heap has been initialized. "); 
    }
}

/// buddy-system 内核内存空间控制器
#[deprecated]
pub mod cutie_heap {
    pub struct CutieHeap; 

    unsafe impl alloc::alloc::GlobalAlloc for CutieHeap {
        unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
            unimplemented!()
        }

        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
            unimplemented!()
        }
    }
}