use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::VirtAddr;

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

// pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     translate_addr_inner(addr, physical_memory_offset)
// }
//
// fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     use x86_64::registers::control::Cr3;
//     use x86_64::structures::paging::page_table::FrameError;
//
//     let (level_4_table_frame, _) = Cr3::read();
//     let t_idx = [addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()];
//     let mut frame = level_4_table_frame;
//
//     for &index in &t_idx {
//         // convert the frame into a page table reference
//         let virt = physical_memory_offset + frame.start_address().as_u64();
//         let table_ptr: *const PageTable = virt.as_ptr();
//         let table = unsafe { &*table_ptr };
//
//         // read the page table entry and update `frame`
//         let entry = &table[index];
//         frame = match entry.frame() {
//             Ok(frame) => frame,
//             Err(FrameError::FrameNotPresent) => return None,
//             Err(FrameError::HugeFrame) => panic!("Huge pages not supported"),
//         };
//     }
//     Some(frame.start_address() + u64::from(addr.page_offset()))
// }
