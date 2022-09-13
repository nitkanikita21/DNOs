pub mod gdt;
pub mod interrupts;

pub fn init() {
    gdt::gdt::init();
    interrupts::init_idt();
}
