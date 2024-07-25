use conquer_once::spin::OnceCell;
use spin::Mutex;
use x2apic::lapic::{LocalApic, LocalApicBuilder};
use x86_64::instructions::port::Port;
use x86_64::PhysAddr;

pub static mut LAPIC: OnceCell<Mutex<LApic>> = OnceCell::uninit();

pub struct LApic {
    addr: u64,
    lapic: Option<LocalApic>,
}

impl LApic {
    pub fn new(addr: u64) -> Self {
        Self {
            addr: crate::memory::physical_to_virtual(PhysAddr::new(addr)).as_u64(),
            lapic: None,
        }
    }

    pub fn init(&mut self) {
        unsafe {
            let mut cmd_8259a = Port::<u8>::new(0x20);
            let mut data_8259a = Port::<u8>::new(0x21);
            let mut cmd_8259b = Port::<u8>::new(0xa0);
            let mut data_8259b = Port::<u8>::new(0xa1);

            let mut spin_port = Port::<u8>::new(0x80);
            let mut spin = || spin_port.write(0);

            cmd_8259a.write(0x11);
            cmd_8259b.write(0x11);
            spin();

            data_8259a.write(0xf8);
            data_8259b.write(0xff);
            spin();

            data_8259a.write(0b100);
            spin();

            data_8259b.write(0b10);
            spin();

            data_8259a.write(0x1);
            data_8259b.write(0x1);
            spin();

            data_8259a.write(u8::MAX);
            data_8259b.write(u8::MAX);
        }

        self.lapic = LocalApicBuilder::default()
            .timer_vector(32)
            .error_vector(51)
            .spurious_vector(0xff)
            .set_xapic_base(self.addr)
            .build()
            .ok();
    }

    pub fn enable(&mut self) {
        unsafe {
            self.lapic.as_mut().unwrap().enable();
        }
    }

    pub fn disable(&mut self) {
        unsafe {
            self.lapic.as_mut().unwrap().disable();
        }
    }

    pub fn end_interrupts(&mut self) {
        unsafe {
            self.lapic.as_mut().unwrap().end_of_interrupt();
        }
    }

    pub fn id(&self) -> u32 {
        unsafe { self.lapic.as_ref().unwrap().id() }
    }
}

pub fn init_lapic(lapic_addr: u64) {
    unsafe {
        LAPIC.init_once(|| Mutex::new(LApic::new(lapic_addr)));
        LAPIC.get().unwrap().lock().init();
    }
}
