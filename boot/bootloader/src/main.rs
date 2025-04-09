#![no_std]
#![no_main]

mod gdt;
mod modes;

#[no_mangle]
pub extern "C" fn main() -> ! {}
