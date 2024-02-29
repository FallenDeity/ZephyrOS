use core::ptr::{read_volatile, write_volatile};

use conquer_once::spin::OnceCell;
use spin::Mutex;
use x86_64::PhysAddr;

use super::reg::Register;

pub static mut LAPIC: OnceCell<Mutex<LocalApic>> = OnceCell::uninit();

pub struct LocalApic {
    addr: u64,
}

impl LocalApic {
    pub fn new(addr: u64) -> Self {
        Self {
            addr: crate::memory::physical_to_virtual(PhysAddr::new(addr)).as_u64(),
        }
    }

    unsafe fn read(&self, reg: Register) -> u32 {
        read_volatile((self.addr + reg as u64) as *const u32)
    }

    unsafe fn write(&mut self, reg: Register, value: u32) {
        write_volatile((self.addr + reg as u64) as *mut u32, value);
        self.read(Register::Id); // wait for write to finish, by reading
    }

    pub fn init(&mut self) {
        unsafe {
            self.write(Register::SpuriousInterruptVector, 0x100 | 39);
            self.write(Register::TimerDivideConfiguration, 0xb);
            self.write(Register::TimerLocalVectorTableEntry, 0x20000 | 32);
            self.write(Register::TimerInitialCount, 10000000);
            self.write(Register::LocalInterrupt0VectorTableEntry, 0x10000);
            self.write(Register::LocalInterrupt1VectorTableEntry, 0x10000);

            if (self.read(Register::Version) >> 16 & 0xFF) >= 4 {
                self.write(Register::PerformanceCounterLocalVectorTableEntry, 0x10000);
            }

            self.write(Register::ErrorVectorTableEntry, 51);
            self.write(Register::ErrorStatus, 0);
            self.write(Register::ErrorStatus, 0);

            self.write(Register::EndOfInterrupt, 0);
            self.write(Register::InterruptCommandHigh, 0);
            self.write(Register::InterruptCommandLow, 0x80000 | 0x500 | 0x8000);

            while self.read(Register::InterruptCommandLow) & 0x1000 != 0 {}
            self.write(Register::TaskPriority, 0);
        }
    }

    pub fn end_interrupts(&mut self) {
        unsafe {
            self.write(Register::EndOfInterrupt, 0);
        }
    }
}

pub fn init_lapic(lapic_addr: u64) {
    unsafe {
        LAPIC.init_once(|| Mutex::new(LocalApic::new(lapic_addr)));
        LAPIC.get().unwrap().lock().init();
    }
}
