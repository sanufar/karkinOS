#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::BootInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_ref() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer();
    }
    loop {}
}


