use crate::test_framework::{custom_assert};
use crate::{print, println};

#[test_case]
fn page_allocation_test_runner(){
    println!("\n---------  Running Page Allocation tests  ---------\n");
   //  fake_test_1();
   //  fake_test_2();
   //fake_test_2(); // This is how you filter certain tests

}

fn fake_test_1(){
   let suc_msg = "Fake Test 1 passed [OK]";
   let fail_msg = "Fake Test 1 failed [FAIL]";
   custom_assert(4, 5, suc_msg, fail_msg);
}

fn fake_test_2(){
   let suc_msg = "Fake Test 2 passed [OK]";
   let fail_msg = "Fake Test 2 failed [FAIL]";
   custom_assert(4, 4, suc_msg, fail_msg); 
}

fn fake_test_3(){
   let suc_msg = "Fake Test 1 passed [OK]";
   let fail_msg = "Fake Test 1 failed [FAIL]";
   custom_assert(4, 3, suc_msg, fail_msg);
}
