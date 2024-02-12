mod dummy_allocator;

use linked_list_allocator::LockedHeap;
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB};
use x86_64::VirtAddr;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() -> Result<(), MapToError<Size4KiB>> {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    let heap_end = heap_start + HEAP_SIZE as u64 - 1u64;
    let heap_start_page = Page::containing_address(heap_start);
    let heap_end_page = Page::containing_address(heap_end);

    let page_range = { Page::range_inclusive(heap_start_page, heap_end_page) };

    let mut frame_allocator_guard = super::frame_alloc::FRAME_ALLOCATOR.lock();
    let mut mapper = super::PAGE_MAP.lock();

    #[allow(irrefutable_let_patterns)]
    if let frame_allocator = frame_allocator_guard.get_mut() {
        for page in page_range {
            let frame = frame_allocator
                .allocate_frame()
                .ok_or(MapToError::FrameAllocationFailed)?;
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
            unsafe {
                mapper.map_to(page, frame, flags, frame_allocator)?.flush();
            }
        }
    }
    unsafe {
        ALLOCATOR.lock().init(heap_start.as_mut_ptr(), HEAP_SIZE);
    }

    Ok(())
}
