use core::arch::asm;

use crate::gdt::GDT;

pub fn enable_a20() {}
pub fn unreal_mode() {
    let ds: u16;
    let ss: u16;
    unsafe {
        asm!("mov {0:x}, ds", "mov {1:x}, ss", out(reg) ds, out(reg) ss);
    }

    protected_mode();

    unsafe {
        asm!(
            "mov eax, cr0",
            "and eax, 0xFFFFFFFE",
            "mov cr0, eax",
            "jmp 0:1f",
            "1:",
            options(nostack)
        );

        asm!(
            "mov ds, {0:x}",
            "mov ss, {1:x}",
            "sti",
            in(reg) ds,
            in(reg) ss
        );
    }
}
pub fn protected_mode() {
    unsafe {
        asm!("cli");
    }

    GDT.load();

    unsafe {
        asm!("mov eax, cr0", "or eax, 1", "mov cr0, eax");

        asm!("ljmp ${0:w}, $1f", "1:", in(reg) GDT.code_segment_selector());

        asm!(
                ".code32",

                "mov ax, {0:w}",
                "mov ds, ax",
                "mov ss, ax",
                "mov es, ax",
                "mov fs, ax",
                "mov gs, ax",
                in(reg) GDT.data_segment_selector()
        );
    }
}
pub fn long_mode() {}
