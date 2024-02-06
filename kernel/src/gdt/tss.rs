use core::ptr::addr_of;

use spin::Lazy;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const DEBUG_IST_INDEX: u16 = 1;
pub const NON_MASKABLE_INTERRUPT_IST_INDEX: u16 = 2;

pub static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    let frames = [
        0,
        DOUBLE_FAULT_IST_INDEX,
        DEBUG_IST_INDEX,
        NON_MASKABLE_INTERRUPT_IST_INDEX,
    ];
    for (i, &_frame) in frames.iter().enumerate() {
        tss.interrupt_stack_table[i] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { addr_of!(STACK) });
            stack_start + STACK_SIZE
        };
    }
    tss
});
