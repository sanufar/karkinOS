#![no_std]
#![no_main]

mod framebuffer;

use core::panic::PanicInfo;
use bootloader_api::BootInfo;
use bootloader_api::info::FrameBufferInfo;

use conquer_once::spin::OnceCell;
use bootloader_x86_64_common::logger::LockedLogger;
use framebuffer::FramebufferDisplay;

use embedded_graphics::geometry::Dimensions;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    };
use embedded_graphics::text::{Alignment, Text};
use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb888;


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
    //init_logger(raw_frame_buffer, frame_buffer_info);

    let mut display = FramebufferDisplay::new(frame_buffer_struct);

    let character_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(255,255,255));

    let text = "embedded-graphics";
    Text::with_alignment(
        text,
        display.bounding_box().center() + Point::new(0, 15),
        character_style,
        Alignment::Center,
    )
    .draw(&mut display).unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

