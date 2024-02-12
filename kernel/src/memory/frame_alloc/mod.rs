pub mod bootinfo_allocator;
pub mod empty_allocator;

use bootloader_api::info::MemoryRegions;
use spin::{Lazy, Mutex};
use x86_64::structures::paging::{FrameAllocator, PhysFrame};

use crate::memory::frame_alloc::bootinfo_allocator::BootInfoFrameAllocator;

pub static FRAME_ALLOCATOR: Lazy<Mutex<BootInfoFrameAllocator>> =
    Lazy::new(|| Mutex::new(BootInfoFrameAllocator::new()));

/// # Safety
/// This function is unsafe because the caller must guarantee that the memory_regions is valid.
pub unsafe fn init_memory_regions(memory_regions: &'static MemoryRegions) {
    FRAME_ALLOCATOR.lock().init(memory_regions);
}

pub fn allocate_frame() -> Option<PhysFrame> {
    FRAME_ALLOCATOR.lock().allocate_frame()
}
