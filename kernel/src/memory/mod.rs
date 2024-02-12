use spin::{Lazy, Mutex};
use x86_64::structures::paging::OffsetPageTable;

use crate::PHYSICAL_MEMORY_OFFSET;

pub mod alloc;
pub mod frame_alloc;
pub mod page;

pub static PAGE_MAP: Lazy<Mutex<OffsetPageTable<'static>>> = Lazy::new(|| {
    let physical_memory_offset = *PHYSICAL_MEMORY_OFFSET.get().unwrap();
    let page_table = unsafe { page::init_page_table(physical_memory_offset) };
    Mutex::new(page_table)
});
