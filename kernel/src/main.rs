#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader_api::info::{BootInfo, FrameBufferInfo};
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;

use crate::framebuffer::renderer::{init_text_renderer, TEXT_RENDERER};

mod framebuffer;

bootloader_api::entry_point!(kernel_main);

#[allow(dead_code)]
pub(crate) static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

#[allow(dead_code)]
pub(crate) fn init_logger(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(buffer, info, true, false));
    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Hello, Kernel Mode!");
}

#[allow(clippy::empty_loop)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(frame_buffer) = boot_info.framebuffer.as_mut() {
        let frame_buffer_info = frame_buffer.info();
        // init_logger(frame_buffer.buffer_mut(), frame_buffer_info);
        // log::debug!("Frame buffer initialized with {:?}", frame_buffer_info);
        init_text_renderer(frame_buffer);
        TEXT_RENDERER.get().unwrap().lock().clear();
        println!("Hello, Kernel Mode!");
        println!("Frame buffer initialized with {:?}", frame_buffer_info);
    }

    let mut x = 0;
    loop {
        println!("Hello, Kernel Mode! {}", x);
        x += 1;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Kernel panic: {:?}", _info);
    loop {}
}
