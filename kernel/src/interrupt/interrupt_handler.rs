use alloc::collections::VecDeque;

use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use spin::mutex::Mutex;
use spin::Lazy;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use crate::renderer::text_renderer;
use crate::{println, serial_println};

const STDIN_BUFFER_SIZE: usize = 10;

pub static STDIN_BUFFER: Lazy<Mutex<VecDeque<u8>>> =
    Lazy::new(|| Mutex::new(VecDeque::with_capacity(STDIN_BUFFER_SIZE)));

fn _set_color(color: Rgb888) {
    text_renderer::TEXT_RENDERER.get().unwrap().lock().set_color(color);
}

macro_rules! interrupt_handler {
    ($name:tt, $info:expr) => {
        pub extern "x86-interrupt" fn $name(stack_frame: InterruptStackFrame) {
            panic!("EXCEPTION: {}\n{:#?}", $info, stack_frame);
        }
    };
}

pub extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    panic!("EXCEPTION: MACHINE CHECK\n{:#?}", stack_frame);
}

interrupt_handler!(divide_by_zero_handler, "DIVIDE BY ZERO");
interrupt_handler!(debug_handler, "DEBUG");
interrupt_handler!(non_maskable_interrupt_handler, "NON MASKABLE INTERRUPT");
interrupt_handler!(overflow_handler, "OVERFLOW");
interrupt_handler!(bound_range_exceeded_handler, "BOUND RANGE EXCEEDED");
interrupt_handler!(invalid_opcode_handler, "INVALID OPCODE");
interrupt_handler!(device_not_available_handler, "DEVICE NOT AVAILABLE");
interrupt_handler!(x87_floating_point_handler, "X87 FLOATING POINT");
interrupt_handler!(simd_floating_point_handler, "SIMD FLOATING POINT");
interrupt_handler!(virtualization_handler, "VIRTUALIZATION");

macro_rules! error_code_interrupt_handler {
    ($name:tt, $info:expr) => {
        pub extern "x86-interrupt" fn $name(stack_frame: InterruptStackFrame, error_code: u64) {
            panic!(
                "EXCEPTION: {} - ERROR CODE: {}\n{:#?}",
                $info, error_code, stack_frame
            );
        }
    };
}

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT - ERROR CODE: {}\n{:#?}",
        error_code, stack_frame
    );
}

error_code_interrupt_handler!(invalid_tss_handler, "INVALID TSS");
error_code_interrupt_handler!(segment_not_present_handler, "SEGMENT NOT PRESENT");
error_code_interrupt_handler!(stack_segment_fault_handler, "STACK SEGMENT FAULT");
error_code_interrupt_handler!(general_protection_fault_handler, "GENERAL PROTECTION FAULT");
error_code_interrupt_handler!(alignment_check_handler, "ALIGNMENT CHECK");
error_code_interrupt_handler!(security_exception_handler, "SECURITY EXCEPTION");

pub extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;

    panic!(
        "EXCEPTION: PAGE FAULT - ERROR CODE: {:?}\nAccessed Address: {:?}\n{:#?}",
        error_code,
        Cr2::read(),
        stack_frame
    );
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    _set_color(Rgb888::RED);
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    _set_color(Rgb888::WHITE);

    serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use crate::interrupt::apic::lapic::LAPIC;

    unsafe {
        LAPIC.get().unwrap().lock().end_interrupts();
    }
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    use crate::interrupt::apic::lapic::LAPIC;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    crate::task::keyboard::add_scancode(scancode);

    unsafe {
        LAPIC.get().unwrap().lock().end_interrupts();
    }
}
