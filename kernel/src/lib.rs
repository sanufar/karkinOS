#![no_std]
#![feature(abi_x86_interrupt)]

pub mod framebuffer;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod logger;

#[cfg(feature = "kerntest")]
pub mod tests;
