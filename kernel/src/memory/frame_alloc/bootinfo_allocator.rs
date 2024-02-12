use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};
use x86_64::PhysAddr;

pub struct BootInfoFrameAllocator {
    memory_regions: Option<&'static MemoryRegions>,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub fn new() -> Self {
        Self {
            memory_regions: None,
            next: 0,
        }
    }

    /// # Safety
    /// This function is unsafe because the caller must guarantee that the memory_regions is valid.
    pub unsafe fn init(&mut self, memory_regions: &'static MemoryRegions) {
        self.memory_regions.replace(memory_regions);
    }

    pub fn get_mut(&mut self) -> &mut Self {
        self
    }

    fn usable_frame(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_regions.expect("memory region not init").iter();
        let usable_regions = regions.filter(|r| r.kind == MemoryRegionKind::Usable);
        let addr_ranges = usable_regions.map(|r| r.start..r.end);
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096)); // 4KiB
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

impl Default for BootInfoFrameAllocator {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frame().nth(self.next);
        self.next += 1;
        frame
    }
}

unsafe impl Send for BootInfoFrameAllocator {}
unsafe impl Sync for BootInfoFrameAllocator {}
