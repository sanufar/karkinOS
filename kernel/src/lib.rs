#![no_std]
#![feature(abi_x86_interrupt)]

pub mod framebuffer;
pub mod gdt;
pub mod idt;
pub mod logger;
pub mod tests;
pub mod interrupts;

use core::sync::atomic::{AtomicUsize, Ordering};
use paste::paste;


#[derive(Copy, Clone)]
pub struct TestEntry {
    pub name: &'static str,
    pub func: fn(),
}

const MAX_TESTS: usize = 128;
static mut TESTS: [Option<TestEntry>; MAX_TESTS] = [None; MAX_TESTS];
static NEXT: AtomicUsize = AtomicUsize::new(0);

pub fn add(name: &'static str, f: fn()) {
    let idx = NEXT.fetch_add(1, Ordering::Relaxed);
    assert!(idx < MAX_TESTS, "MAX_TESTS overflow");
    unsafe { TESTS[idx] = Some(TestEntry { name, func: f }) };
}

pub fn run_all() {
    for slot in unsafe { &TESTS[..NEXT.load(Ordering::Relaxed)] } {
        if let Some(test) = slot { 
            log::info!("Running test: {}", test.name);
            (test.func)();
            log::info!("Test '{}' [ok]", test.name);
        }
    }
}

#[macro_export]
macro_rules! ktest {
    (fn $name:ident() $body:block) => {
        paste! {
            pub fn $name() $body

            #[allow(non_snake_case)]
            pub fn [<__register_ $name>]() {
                $crate::add(stringify!($name), $name);
            }
        }
    };
}

#[macro_export]
macro_rules! register_tests {
    ( $( $test:ident ),* $(,)? ) => {
        paste! {
            pub fn __register_all() {
                $( [<__register_ $test>](); )*
            }
        }
    };
}

#[macro_export]
macro_rules! collect_tests {
    ( $( $module:ident ),* $(,)? ) => {
        pub fn _init_tests() {
            $( $module::__register_all(); )*
        }
    };
}
