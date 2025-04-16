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

    tests::init_tests();
    tests::run_all();  

    #[cfg(feature = "kerntest")]
    {
        init_tests();
        tests::run_all_tests();
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


#[cfg(feature = "kerntest")]
fn example_test() {
    assert_eq!(1, 1);
}

#[cfg(feature = "kerntest")]
fn test_something() {
    assert_ne!(0, 1);
}

#[cfg(feature = "kerntest")]
pub fn init_tests() {
    tests::register_test("test_something", test_something);
    tests::register_test("example_test", example_test);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    log::error!("{}", _info);
    loop {}
}
