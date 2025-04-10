#![no_std]
#![no_main]

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;
use core::str;

use boot_utils::disk::*;
use boot_utils::print;

const BOOTLOADER_LBA: u64 = 2048;
const BOOTLOADER_SECTOR_SIZE: u16 = 64;

global_asm!(include_str!("boot.asm"));

// We define this value in our linker script.
extern "C" {
    static _bootloader_start: u16;
}

#[no_mangle]
pub extern "C" fn main() -> ! {

    print::raw_print("\r\n\0");
    print::raw_print("[*] Starting Stage 1 of boot...\r\n\0");
    print::raw_print("[*] Loading bootloader...\r\n\0");

    let bootloader_start: *const u16 = unsafe { &_bootloader_start };

    if let addr = bootloader_start as u16 == 0x7e00 {
    print::raw_print("[*] Bootloader address is correct (0x7e00)\r\n\0");
    }

    let offset_target = bootloader_start as u16;

    let mut disk = DiskReader::new(offset_target, BOOTLOADER_LBA);

    if !disk.read_sectors(BOOTLOADER_SECTOR_SIZE) {
        print::raw_print("[!] Failed to load bootloader.\r\n\0");
        loop {} // Halt on error
    }

    print::raw_print("[*] Successfully loaded bootloader.\r\n\0");

    jump(bootloader_start);

    loop {}
}


fn jump(address: *const u16) {
    unsafe {
        asm!(
            "jmp {0:x}",
            in(reg) address as u16,
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
