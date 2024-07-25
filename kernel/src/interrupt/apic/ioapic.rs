use alloc::vec;

use conquer_once::spin::OnceCell;
use spin::Mutex;
use x2apic::ioapic::{IoApic, IrqMode, RedirectionTableEntry};
use x86_64::PhysAddr;

use super::lapic;
use crate::memory;

pub static mut IOAPIC: OnceCell<Mutex<vec::Vec<IOApic>>> = OnceCell::uninit();

pub struct IOApic {
    addr: u64,
    ioapic: Option<IoApic>,
}

impl IOApic {
    pub fn new(addr: u64) -> Self {
        Self {
            addr: memory::physical_to_virtual(PhysAddr::new(addr)).as_u64(),
            ioapic: None,
        }
    }

    pub fn init(&mut self) {
        self.ioapic = unsafe { Option::from(IoApic::new(self.addr)) };
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn enable(&mut self) {
        self.ioapic.as_mut().unwrap().init(32);
        let mut entry = RedirectionTableEntry::default();
        entry.set_mode(IrqMode::Fixed);
        entry.set_vector(33);
        entry.set_dest(lapic::LAPIC.get().unwrap().lock().id() as u8);

        self.ioapic.as_mut().unwrap().set_table_entry(1, entry);
        self.ioapic.as_mut().unwrap().enable_irq(1);
    }

    pub fn get_ioapic(&self) -> &IoApic {
        self.ioapic.as_ref().unwrap()
    }
}

pub fn init_ioapic(ioapic_addr: u64) {
    unsafe {
        if IOAPIC.get().is_none() {
            IOAPIC.init_once(|| Mutex::new(vec![IOApic::new(ioapic_addr)]));
        } else {
            IOAPIC.get().unwrap().lock().push(IOApic::new(ioapic_addr));
        }
    }
}
