#[cfg(feature = "kerntest")]
const MAX_TESTS: usize = 128;

#[cfg(feature = "kerntest")]
#[derive(Copy, Clone)]
pub struct TestEntry {
    pub name: &'static str,
    pub func: fn(),
}

#[cfg(feature = "kerntest")]
static mut TEST_REGISTRY: [Option<TestEntry>; MAX_TESTS] = [None; MAX_TESTS];

#[cfg(feature = "kerntest")]
static mut TEST_COUNT: usize = 0;

#[cfg(feature = "kerntest")]
pub fn register_test(name: &'static str, func: fn()) {
    unsafe {
        if TEST_COUNT < MAX_TESTS {
            TEST_REGISTRY[TEST_COUNT] = Some(TestEntry { name, func });
            TEST_COUNT += 1;
        } else {
            // Registry full
            log::error!("Test registry full, cannot add test: {}", name);
        }
    }
}

#[cfg(feature = "kerntest")]
pub fn run_all_tests() {
    let count = unsafe { TEST_COUNT };
    log::info!("Running {} kernel tests", count);

    unsafe {
        for i in 0..count {
            if let Some(test) = TEST_REGISTRY[i] {
                log::info!("Running test: {}", test.name);
                (test.func)();
                log::info!("Test '{}'\t [ok]", test.name);
            }
        }
    }

    log::info!("All tests complete");
}
