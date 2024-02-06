#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupt;
pub mod renderer;

use bootloader_api::BootInfo;

pub fn init(boot_info: &'static mut BootInfo) {
    let frame_buffer = boot_info.framebuffer.as_mut().unwrap();
    let frame_buffer_info = frame_buffer.info();
    renderer::text_renderer::init_text_renderer(frame_buffer);
    renderer::text_renderer::TEXT_RENDERER.get().unwrap().lock().clear();
    println!("Frame buffer initialized with {:?}", frame_buffer_info);
    gdt::init_gdt();
    println!("GDT Initialized");
    interrupt::init_idt();
    println!("Hardware Interrupts Initialized");
}
