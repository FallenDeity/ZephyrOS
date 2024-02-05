#![no_std]
#![no_main]

use core::convert::Infallible;
use core::panic::PanicInfo;

use bootloader_api::info::{BootInfo, FrameBufferInfo};
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;

mod framebuffer;

bootloader_api::entry_point!(kernel_main);

pub(crate) static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

pub(crate) fn init_logger(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(buffer, info, true, false));
    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Hello, Kernel Mode!");
}

#[allow(clippy::empty_loop)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let frame_buffer_info = framebuffer.info();
        init_logger(framebuffer.buffer_mut(), frame_buffer_info);
        log::info!("Logger initialized");
        log::debug!("FrameBufferInfo: {:?}", frame_buffer_info);
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[allow(dead_code)]
fn infallible<T>(v: Infallible) -> T {
    match v {}
}
