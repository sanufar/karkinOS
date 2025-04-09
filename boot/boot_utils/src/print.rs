use core::{
    arch::asm,
    fmt::{self},
};

pub struct Printer {}

pub static mut PRINTER: Printer = Printer {};

impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}

impl Printer {
    fn print(&self, message: &str) {
        raw_print(message);
    }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::print!("{}\r\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        PRINTER.write_fmt(args).unwrap();
    }
}

pub fn raw_print(message: &str) {
    let offset: u16 = message.as_ptr() as u16;

    unsafe {
        asm!(
            "mov si, {0:x}",

            "2:",
                "lodsb",
                "test al, al",
                "jz 3f",

                "mov ah, 0x0E",
                "mov bh, 0",
                "int 0x10",

                "jmp 2b",

            "3:",

            in(reg) offset,

            clobber_abi("C"),
        )
    }
}

pub fn clear() {
    unsafe {
        asm!("mov ah, 0x00", "mov al, 0x03", "int 0x10");
    }
}
