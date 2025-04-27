use crate::idt::*;
use conquer_once::spin::Lazy;

pub fn init_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt
}

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(init_idt);

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log::info!("EXCEPTION: BREAKPOINT");
}
