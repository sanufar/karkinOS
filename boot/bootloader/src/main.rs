#![no_std]
#![no_main]

mod gdt;
mod modes;

use boot_utils::disk::DiskReader;
use modes::protected_mode;
use modes::unreal_mode;

use core::panic::PanicInfo;

const KERNEL_LBA: u64 = 4096; 

const KERNEL_SIZE: u16 = 2048;

const KERNEL_BUFFER: u16 = 0xbe00; 
const KERNEL_TARGET: u32 = 0x0010_0000; 

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    boot_utils::println!("PANIC! Info: {}", info);

    loop {}
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    boot_utils::println!("[*] Starting Stage 2 of bootloader...");

    boot_utils::println!("[*] Enabling the A20 line...");
    match modes::enable_a20() {
        Ok(()) => boot_utils::println!("[*] Successfully enabled A20 line!"),
        Err(err) => boot_utils::println!("[!] ERROR: {}", err),
    }

    boot_utils::println!("[*] Entering unreal mode...");

    unreal_mode();

    boot_utils::println!("[*] Loading kernel...");

    let mut disk_reader = DiskReader::new(KERNEL_BUFFER, KERNEL_LBA);

    if !disk_reader.read_and_copy_sectors(KERNEL_SIZE, KERNEL_TARGET) {
        boot_utils::println!("[!] ERROR: Failed to load kernel into target.");
    };

    boot_utils::println!("[*] Entering protected mode...");

    protected_mode();

    loop {};
}
