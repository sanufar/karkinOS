use crate::*;             

ktest!(fn addition()  { 
    assert_eq!(1 + 1, 2);
    log::info!("Test passed.")
});

ktest!(fn multiply()  {
    assert_eq!(2 * 3, 6); 
    log::info!("Test 2 passed.")
});

register_tests!(addition, multiply);

