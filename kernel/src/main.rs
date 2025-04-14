#![no_std]
#![no_main]

use bootloader_api::BootInfo;
use core::panic::PanicInfo;

use kernel::*;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let frame_buffer_optional = &mut boot_info.framebuffer;
    let frame_buffer_option = frame_buffer_optional.as_mut();
    let frame_buffer_struct = frame_buffer_option.unwrap();
    let frame_buffer_info = frame_buffer_struct.info().clone();
    let raw_frame_buffer = frame_buffer_struct.buffer_mut();
    logger::init(raw_frame_buffer, frame_buffer_info);

    custard();

    loop {}
}

fn custard() {
    #[cfg(feature = "kerntest")]
    {
        log::info!("Running test...");
        assert_eq!(1,1);
        log::info!("[ok]")
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    log::error!("{}", _info);
    loop {}
}


