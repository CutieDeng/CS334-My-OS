use core::alloc::Layout;

#[alloc_error_handler]
fn alloc_error_handler(_: Layout) -> ! {
    panic!("内存分配页异常") 
}

mod allocator;
mod frame_tracker; 

pub use allocator::FRAME_ALLOCATOR; 

type MemoryResult<T> = Result<T, &'static str>; 