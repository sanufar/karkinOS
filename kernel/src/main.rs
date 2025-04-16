#![no_std]
#![no_main]

use bootloader_api::BootInfo;
use core::panic::PanicInfo;
use core::arch::asm;

use kernel::*;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let frame_buffer_optional = &mut boot_info.framebuffer;
    let frame_buffer_option = frame_buffer_optional.as_mut();
    let frame_buffer_struct = frame_buffer_option.unwrap();
    let frame_buffer_info = frame_buffer_struct.info().clone();
    let raw_frame_buffer = frame_buffer_struct.buffer_mut();
    logger::init(raw_frame_buffer, frame_buffer_info);

    #[cfg(feature = "kerntest")]
    {
        tests::init_tests();
        tests::run_all();  
    }

    interrupts::init();

    int3();

    loop {}
}

pub fn int3() {
    unsafe {
        asm!("int3", options(nomem, nostack));
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    log::error!("{}", _info);
    loop {}
}
