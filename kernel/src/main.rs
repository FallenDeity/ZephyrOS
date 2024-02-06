#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use bootloader_api::info::BootInfo;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
#[allow(unused_imports)]
use kernel::{print, println, renderer::text_renderer};

bootloader_api::entry_point!(kernel_main);

#[allow(clippy::empty_loop)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init(boot_info);
    println!("Kernel Initialized...");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    text_renderer::TEXT_RENDERER
        .get()
        .unwrap()
        .lock()
        .set_color(Rgb888::RED);
    println!("Kernel panic: {:?}", _info);
    text_renderer::TEXT_RENDERER
        .get()
        .unwrap()
        .lock()
        .set_color(Rgb888::WHITE);
    loop {}
}
