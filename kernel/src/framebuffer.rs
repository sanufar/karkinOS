#![allow(dead_code)]

use core::char;

use bootloader_api::info::{FrameBuffer, PixelFormat};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    pub position: Position,
    pub color: Color,
}

pub(crate) struct FramebufferDisplay<'f> {
    framebuffer: &'f mut FrameBuffer,
}

impl<'f> FramebufferDisplay<'f> {
    pub fn new(framebuffer: &mut FrameBuffer) -> FramebufferDisplay {
        FramebufferDisplay { framebuffer }
    }

    fn set_pixel(&mut self, position: Position, color: Color) {
        let info = self.framebuffer.info();

        let byte_offset = {
            let y_offset = position.y * info.stride;
            let pixel_offset = y_offset + position.x;

            pixel_offset * info.bytes_per_pixel
        };

        let pixel_buffer = &mut self.framebuffer.buffer_mut()[byte_offset..];

        match info.pixel_format {
            PixelFormat::Rgb => {
                pixel_buffer[0] = color.red;
                pixel_buffer[1] = color.blue;
                pixel_buffer[2] = color.green;
            }
            PixelFormat::Bgr => {
                pixel_buffer[0] = color.blue;
                pixel_buffer[1] = color.green;
                pixel_buffer[2] = color.red;
            }
            PixelFormat::U8 => {
                let gray = (color.red + color.green + color.blue) / 3;
                pixel_buffer[0] = gray;
            }
            other => panic!("Unknown pixel format: {other:?}"),
        }
    }

    fn draw_char(&mut self, char: char, pixel: Pixel) {}
}
