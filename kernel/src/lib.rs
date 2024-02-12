#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod gdt;
pub mod interrupt;
pub mod memory;
pub mod renderer;

use bootloader_api::BootInfo;
use conquer_once::spin::OnceCell;
use x86_64::structures::paging::Page;
use x86_64::VirtAddr;

pub static PHYSICAL_MEMORY_OFFSET: OnceCell<VirtAddr> = OnceCell::uninit();

pub fn init(boot_info: &'static mut BootInfo) {
    serial_println!("Starting kernel initialization...");
    let physical_memory_offset = boot_info.physical_memory_offset.into_option();
    if let Some(offset) = physical_memory_offset {
        PHYSICAL_MEMORY_OFFSET.init_once(|| VirtAddr::new(offset));
    } else {
        panic!("Physical memory offset not found");
    }
    let frame_buffer = boot_info.framebuffer.as_mut().unwrap();
    let frame_buffer_info = frame_buffer.info();
    renderer::text_renderer::init_text_renderer(frame_buffer);
    renderer::text_renderer::TEXT_RENDERER.get().unwrap().lock().clear();
    println!("Frame buffer initialized with {:?}", frame_buffer_info);
    serial_println!("Frame buffer initialized with {:?}", frame_buffer_info);
    println!("Physical memory offset: 0x{:?}", boot_info.physical_memory_offset);
    serial_println!("Physical memory offset: 0x{:?}", boot_info.physical_memory_offset);
    gdt::init_gdt();
    println!("GDT Initialized");
    serial_println!("GDT Initialized");
    interrupt::init_idt();
    println!("Hardware Interrupts Initialized");
    serial_println!("Hardware Interrupts Initialized");
    unsafe {
        memory::frame_alloc::init_memory_regions(&boot_info.memory_regions);
    }
    memory::alloc::init_heap().expect("Heap initialization failed");
    println!("Heap initialized");
    serial_println!("Heap initialized");
    let page = Page::containing_address(VirtAddr::new(0));
    let mut mapper = memory::PAGE_MAP.lock();
    let mut frame_allocator = memory::frame_alloc::FRAME_ALLOCATOR.lock();
    memory::page::_map_kernel_pages(page, &mut mapper, &mut frame_allocator);
    println!("Kernel pages mapped");
    serial_println!("Kernel pages mapped");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
