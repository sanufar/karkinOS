use crate::*;

pub mod math;

collect_tests!(math);

pub use self::_init_tests as init_tests;
pub use self::_run_all as run_all;

// ====================================================================//
//                          IMPL HERE FOR NOW                          //
// ====================================================================//

use core::sync::atomic::{AtomicUsize, Ordering};
use paste;

#[derive(Copy, Clone)]
pub struct TestEntry {
    pub name: &'static str,
    pub func: fn(),
}

const MAX: usize = 128;
static mut TESTS: [Option<TestEntry>; MAX] = [None; MAX];
static NEXT: AtomicUsize = AtomicUsize::new(0);

pub fn add(name: &'static str, f: fn()) {
    let idx = NEXT.fetch_add(1, Ordering::Relaxed);
    assert!(idx < MAX, "increase MAX");
    unsafe { TESTS[idx] = Some(TestEntry { name, func: f }) };
}

pub fn _run_all() {
    for slot in unsafe { &TESTS[..NEXT.load(Ordering::Relaxed)] } {
        if let Some(t) = slot {
            log::info!("Running test: {}", t.name);
            (t.func)();
            log::info!("Test '{}'    [ok]", t.name);
        }
    }
}

#[macro_export]
macro_rules! ktest {
    (fn $name:ident() $body:block) => {
        paste::paste! {
            pub fn $name() $body

            #[allow(non_snake_case)]
            pub fn [<__register_ $name>]() {
                crate::tests::add(stringify!($name), $name);
            }
        }
    };
}

#[macro_export]
macro_rules! register_tests {
    ( $( $test:ident ),* $(,)? ) => {
        paste::paste! {
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
