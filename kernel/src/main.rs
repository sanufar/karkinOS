#![no_std]
#![no_main]

mod framebuffer;

use bootloader_api::BootInfo;
use bootloader_api::info::FrameBufferInfo;
use core::panic::PanicInfo;

use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;
use framebuffer::FramebufferDisplay;

pub(crate) static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

pub(crate) fn init_logger(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(buffer, info, true, true));
    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Hello, Kernel Mode!");
}

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let frame_buffer_optional = &mut boot_info.framebuffer;
    let frame_buffer_option = frame_buffer_optional.as_mut();
    let frame_buffer_struct = frame_buffer_option.unwrap();
    let frame_buffer_info = frame_buffer_struct.info().clone();
    let raw_frame_buffer = frame_buffer_struct.buffer_mut();
    init_logger(raw_frame_buffer, frame_buffer_info);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
