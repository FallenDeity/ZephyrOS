#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupt;
pub mod memory;
pub mod renderer;

use bootloader_api::BootInfo;
use x86_64::structures::paging::Translate;

pub fn init(boot_info: &'static mut BootInfo) {
    let frame_buffer = boot_info.framebuffer.as_mut().unwrap();
    let frame_buffer_info = frame_buffer.info();
    renderer::text_renderer::init_text_renderer(frame_buffer);
    renderer::text_renderer::TEXT_RENDERER.get().unwrap().lock().clear();
    println!("Frame buffer initialized with {:?}", frame_buffer_info);
    serial_println!("Frame buffer initialized with {:?}", frame_buffer_info);
    println!("Physical memory offset: 0x{:?}", boot_info.physical_memory_offset);
    serial_println!("Physical memory offset: 0x{:?}", boot_info.physical_memory_offset);
    let memory_offset: Option<u64> = boot_info.physical_memory_offset.into_option();
    if let Some(offset) = memory_offset {
        let virt = x86_64::VirtAddr::new(offset);
        let mapper = unsafe { memory::page::init_page_table(virt) };
        let addreses = [
            0x10000000000,
            0x10000000000 + 4096,
            0x15000000000,
            0x18000000000,
            0x19000000000,
            offset,
        ];
        for &addr in &addreses {
            let v_addr = x86_64::VirtAddr::new(addr);
            let p_addr = mapper.translate_addr(v_addr);
            println!("{:?} -> {:?}", v_addr, p_addr);
            serial_println!("{:?} -> {:?}", v_addr, p_addr);
        }
    } else {
        println!("Physical memory offset not found");
        serial_println!("Physical memory offset not found");
        panic!("Physical memory offset not found");
    }
    gdt::init_gdt();
    println!("GDT Initialized");
    serial_println!("GDT Initialized");
    interrupt::init_idt();
    println!("Hardware Interrupts Initialized");
    serial_println!("Hardware Interrupts Initialized");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
