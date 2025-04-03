#![no_std]
#![no_main]

use core::arch::global_asm;
use core::arch::asm;
use core::panic::PanicInfo;

use disk::DiskReader;

mod disk;

const BOOTLOADER_LBA: u64 = 2048;
const BOOTLOADER_SECTOR_SIZE: u16 = 64;

global_asm!(include_str!("boot.asm"));

// We define this value in our linker script.
extern "C" {
    static _bootloader_start: u16;
}


#[no_mangle]
pub extern "C" fn main() -> ! {
    clear();

    print("[*] Starting Stage 1 of boot...\r\n\0");
    print("[*] Loading bootloader...\r\n\0");

    let bootloader_start: *const u16 = unsafe {&_bootloader_start};

    let offset_target = bootloader_start as u16;
    let mut disk = DiskReader::new(offset_target, BOOTLOADER_LBA);

    disk.read_sectors(BOOTLOADER_SECTOR_SIZE);

    jump(bootloader_start);

    loop {};
    
}


fn print(message: &str) {

    let offset: u16 = message.as_ptr() as u16;

    unsafe {
        asm!(
                "mov si, {0:x}",
            
            // Label 'print_loop':
            "2:",
                // Load a byte from DS:SI into AL, increment SI
                "lodsb",
                // Check if AL is zero (end of string)
                "test al, al",
                // If zero, jump to 'done'
                "jz 3f",

                // Set up BIOS Teletype (AH=0x0E) to print AL
                "mov ah, 0x0E",
                "mov bh, 0",
                "int 0x10",

                // Jump back to 'print_loop'
                "jmp 2b",

            // Label 'done':
            "3:",

            // Pass the offset as input
            in(reg) offset,
            
            clobber_abi("C"),
        )
}
}

fn clear() {
    unsafe {
        asm!("mov ah, 0x00", "mov al, 0x03", "int 0x10");
    }
}

fn jump(address: *const u16) {
    unsafe {
        asm!("jmp {0:x}", in(reg) address as u16);
    }
}

#[no_mangle]
pub extern "C" fn bootload_error() -> ! {
    print("[!] Failed to load bootloader.");
    loop {};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {};
}
