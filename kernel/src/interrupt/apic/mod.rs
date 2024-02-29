use acpi::{AcpiTables, InterruptModel};

pub mod ioapic;
pub mod lapic;
pub mod reg;
pub mod rsdp;

pub fn init(rsdp_addr: &u64) {
    let tables = unsafe { AcpiTables::from_rsdp(rsdp::Handler, *rsdp_addr as usize).unwrap() };
    let platform_info = tables.platform_info().unwrap();
    let interrupt_model = platform_info.interrupt_model;

    if let InterruptModel::Apic(apic) = interrupt_model {
        let lapic_physical_address: u64 = apic.local_apic_address;
        lapic::init_lapic(lapic_physical_address);
        for i in apic.io_apics.iter() {
            crate::println!("{:?}", i);
        }
    }
}
