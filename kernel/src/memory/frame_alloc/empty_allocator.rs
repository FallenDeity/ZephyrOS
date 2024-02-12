use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};

pub struct EmptyAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}
