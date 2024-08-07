#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::info::BootInfo;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use kernel::renderer::text_renderer;
use kernel::{println, serial_println};

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

bootloader_api::entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

#[allow(clippy::empty_loop)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init(boot_info);
    println!("Kernel Initialized...");
    serial_println!("Kernel Initialized...");

    println!("Initializing task executor...");
    serial_println!("Initializing task executor...");
    kernel::task::init_executor();
    println!("Task executor initialized");
    serial_println!("Task executor initialized");

    kernel::hlt_loop();
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

    serial_println!("Kernel panic: {:?}", _info);
    kernel::hlt_loop();
}
