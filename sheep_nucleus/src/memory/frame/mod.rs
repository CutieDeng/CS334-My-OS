mod allocator;
mod frame_tracker; 

pub use allocator::FRAME_ALLOCATOR; 

type MemoryResult<T> = Result<T, &'static str>; 