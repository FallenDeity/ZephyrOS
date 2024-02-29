pub mod apic;
mod interrupt_handler;
mod interrupts;

use spin::Lazy;
use x86_64::structures::idt::InterruptDescriptorTable;

use super::gdt;
use crate::interrupt::interrupts::InterruptIndex;

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    // TODO: Add keyboard interrupt
    let mut idt = InterruptDescriptorTable::new();
    idt.divide_error
        .set_handler_fn(interrupt_handler::divide_by_zero_handler);
    idt.debug.set_handler_fn(interrupt_handler::debug_handler);
    idt.non_maskable_interrupt
        .set_handler_fn(interrupt_handler::non_maskable_interrupt_handler);
    idt.overflow.set_handler_fn(interrupt_handler::overflow_handler);
    idt.bound_range_exceeded
        .set_handler_fn(interrupt_handler::bound_range_exceeded_handler);
    idt.invalid_opcode
        .set_handler_fn(interrupt_handler::invalid_opcode_handler);
    idt.device_not_available
        .set_handler_fn(interrupt_handler::device_not_available_handler);
    idt.x87_floating_point
        .set_handler_fn(interrupt_handler::x87_floating_point_handler);
    idt.simd_floating_point
        .set_handler_fn(interrupt_handler::simd_floating_point_handler);
    idt.virtualization
        .set_handler_fn(interrupt_handler::virtualization_handler);
    idt.invalid_tss.set_handler_fn(interrupt_handler::invalid_tss_handler);
    idt.segment_not_present
        .set_handler_fn(interrupt_handler::segment_not_present_handler);
    idt.stack_segment_fault
        .set_handler_fn(interrupt_handler::stack_segment_fault_handler);
    idt.general_protection_fault
        .set_handler_fn(interrupt_handler::general_protection_fault_handler);
    idt.page_fault.set_handler_fn(interrupt_handler::page_fault_handler);
    idt.alignment_check
        .set_handler_fn(interrupt_handler::alignment_check_handler);
    idt.security_exception
        .set_handler_fn(interrupt_handler::security_exception_handler);
    idt.breakpoint.set_handler_fn(interrupt_handler::breakpoint_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(interrupt_handler::double_fault_handler)
            .set_stack_index(gdt::tss::DOUBLE_FAULT_IST_INDEX);
    }
    idt.machine_check
        .set_handler_fn(interrupt_handler::machine_check_handler);
    idt[InterruptIndex::Timer.as_usize()].set_handler_fn(interrupt_handler::timer_interrupt_handler);
    idt
});

pub fn init_idt() {
    IDT.load();
}

pub fn enable_interrupts() {
    x86_64::instructions::interrupts::enable();
}

pub fn disable_interrupts() {
    x86_64::instructions::interrupts::disable();
}
