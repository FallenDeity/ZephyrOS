use x86_64::structures::paging::{Mapper, OffsetPageTable, Page, PageTable, PhysFrame};
use x86_64::{PhysAddr, VirtAddr};

use crate::memory::frame_alloc::bootinfo_allocator::BootInfoFrameAllocator;
use crate::println;

unsafe fn _get_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let physical_address = level_4_table_frame.start_address();
    let virtual_address = physical_memory_offset + physical_address.as_u64();
    let page_table_ptr: *mut PageTable = virtual_address.as_mut_ptr();
    &mut *page_table_ptr
}

/// # Safety
/// This function is unsafe because the caller must guarantee that the physical_memory_offset is correct.
pub unsafe fn init_page_table(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = unsafe { _get_level_4_table(physical_memory_offset) };
    unsafe { OffsetPageTable::new(level_4_table, physical_memory_offset) }
}

#[allow(dead_code)]
pub fn display_page_table(page_table: &PageTable) {
    for (i, entry) in page_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("Entry {}: {:?}", i, entry);
        }
    }
}

pub fn _map_kernel_pages(page: Page, mapper: &mut OffsetPageTable, frame_allocator: &mut BootInfoFrameAllocator) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(page.start_address().as_u64()));
    let flags = Flags::PRESENT | Flags::WRITABLE;
    let map_to_result = unsafe { mapper.map_to(page, frame, flags, frame_allocator) };
    map_to_result.expect("Failed to map kernel pages").flush();
}
