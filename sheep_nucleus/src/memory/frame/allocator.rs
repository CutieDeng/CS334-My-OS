use spin::Mutex; 
use algorithm::AllocatorImpl; 
use super::super::{PhysicalAddress, PhysicalPageNumber, Range}; 
use crate::memory; 
use super::frame_tracker::FrameTracker; 

lazy_static::lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>> = 
        Mutex::new (
            FrameAllocator::new (
                Range::from(
                    PhysicalPageNumber::ceil(PhysicalAddress::from(*memory::KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(memory::MEMORY_END_ADDRESS)
                )
            )
        ); 
}

use algorithm::Allocator; 

/// 基于线段树的帧分配 / 回收
pub struct FrameAllocator<T: Allocator> {
    /// 可用区间的起始
    start_ppn: PhysicalPageNumber,
    /// 分配器
    allocator: T,
}

use super::MemoryResult; 

impl<T: Allocator> FrameAllocator<T> {
    /// 创建对象
    pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        FrameAllocator {
            start_ppn: range.into().start,
            allocator: T::new(range.into().len()),
        }
    }

    /// 分配帧，如果没有剩余则返回 `Err`
    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        self.allocator
            .alloc()
            .ok_or("no available frame to allocate")
            .map(|offset| FrameTracker(self.start_ppn + offset))
    }

    /// 将被释放的帧添加到空闲列表的尾部
    ///
    /// 这个函数会在 [`FrameTracker`] 被 drop 时自动调用，不应在其他地方调用
    pub(super) fn dealloc(&mut self, frame: &FrameTracker) {
        self.allocator.dealloc(frame.page_number() - self.start_ppn);
    }
}