//! THis module defines the functions used to run test functions across the crate   
//! It does not define the actual test functions    
//! It defines how the tests get handled 
//! 
//! If you define a test module, you should define it using the following format.   
//! Future James, kindly document this work... You twat
//! 
//! ### Example of a module. Call it module_x   
//! ```rust
//! #![cfg(test)]
//! use crate::{print, println};
//! use crate::test_framework::custom_assert;
//!
//! #[test_case]
//! fn module_x_test_(){
//!     println!("---------  Running trash tests  ---------\n");
//!     fake_test_1();
//!     fake_test_2();
//!     //fake_test_2(); // This is how you filter certain tests
//!
//! }
//!
//! fn fake_test_1(){
//!    let suc_msg = "Fake Test 1 passed [OK]";
//!    let fail_msg = "Fake Test 1 failed [FAIL]";
//!    custom_assert(4, 5, suc_msg, fail_msg);
//! }
//!
//! fn fake_test_2(){
//!    println!("I am a trash test 2"); 
//! }
//!
//! fn fake_test_3(){
//!    println!("I am a trash test 3"); 
//! }
//! ```

use crate::{print, println};
use core::fmt::Debug;


#[cfg(test)] // make cargo test consider this module when you run the 'cargo test' command
/// THis function gets called when you run the cargo test command
/// It reseives all functions across the crate that were declared with the [test_case] tag
pub fn test_runner(all_tests : &[&dyn Fn()]){ // functions accepts group of functions. In this case, every function with the [test_case] tag
    println!("==========  Running Tests in a Module-Wise Manner  =============");
    for test in all_tests{
        test();
    }
}

/// Compares two values. If the values are equal, then the test passes. Else it fails   
/// You have to pass to it the messages to be displayed for each scenario
pub fn custom_assert<T: PartialEq + Debug> ( sure_value: T, test_value: T, suc_msg: &'static str, fail_msg: &'static str){
    let result = sure_value == test_value;
    if result == true { println!("{}", suc_msg);}
    else { println!("{} : ", fail_msg );
           println!("       expected : {:?} but got {:?}", sure_value, test_value);}
}