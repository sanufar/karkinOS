#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub extern "C" fn _start() -> ! {
    loop {};
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {};
}
