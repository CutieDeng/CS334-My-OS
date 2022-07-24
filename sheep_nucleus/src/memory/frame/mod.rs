use core::alloc::Layout;

fn alloc_error_handler(_: Layout) -> ! {
    panic!("内存分配页异常") 
}

mod allocator;
mod frame_tracker; 

pub use allocator::FRAME_ALLOCATOR; 

type MemoryResult<T> = Result<T, &'static str>; 