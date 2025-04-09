use core::arch::asm;

pub fn print(message: &str) {

    let offset: u16 = message.as_ptr() as u16;

    unsafe {
        asm!(
            "mov si, {0:x}",
            
            "2:",
                "lodsb",
                "test al, al",
                "jz 3f",

                "mov ah, 0x0E",
                "mov bh, 0",
                "int 0x10",

                "jmp 2b",

            "3:",

            in(reg) offset,
            
            clobber_abi("C"),
        )
}
}


#[no_mangle]
pub extern "C" fn bootload_error() -> ! {
    print("[!] Failed to load bootloader.\r\n\0");
    loop {};
}
