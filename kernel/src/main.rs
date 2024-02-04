#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader_api::BootInfo;

bootloader_api::entry_point!(kernel_main);

#[allow(clippy::empty_loop)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        for byte in framebuffer.buffer_mut() {
            *byte = 0x90;
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
