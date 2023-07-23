
#![no_std]  // we will no depend on Rust Standard Library and Libc
#![no_main] // we will define our own entry point sequence using the linker + Bootloader
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)] // enable the use of a custom test framework
#![test_runner(crate::test_framework::test_runner)]
#![reexport_test_harness_main = "test_framework_entry_point"] // give the entrypoint a custom name, and add it to the program flow


// mod utilities;
mod drivers;
mod asm;
mod stdout;
mod stdin;
mod memory;
mod test_framework; 


// usage of accessible modules
use core::{arch::asm, panic::PanicInfo};
use core::fmt::Write; // enable the use of Write functions in this scope
use stdin::continuous_keyboard_read;



// defining the function that will always get called after a panic
#[panic_handler]
fn panic_handler (panic_info: &PanicInfo) -> !{
    // make the current CPU_core sleep endlessly and wait for Interrupt
    let panic_location = panic_info.location().unwrap();
    let panic_message = panic_info.message().unwrap();
    println!("Panic occured : in file : {}, line {}, with the message : {:?}",
                                                 panic_location.file(), panic_location.line(), panic_message);
    loop {
       unsafe { asm!("wfi");  }
    }
}


// defining the entry point function
#[no_mangle]
pub extern "C" fn kmain () {
    println!("jhgjgjhg {}", 5);
    memory::init_memory();
    // memory::show_layout();

    let order = memory::check_descriptor_ordering();
    if order == false { println!(">>> ordering of descriptors is messed up... ");}

    let loc_1_address = memory::alloc(5).unwrap();
    let loc_2_address = memory::alloc(10).unwrap();
    memory::show_layout();


    let order = memory::check_descriptor_ordering();
    if order == false { println!(">>> ordering of descriptors is messed up... ");}

    let dealloc_result = memory::dealloc(loc_2_address);
    match  dealloc_result {
        Ok(x) => /*do nothing */{},
        Err(error) => {println!("Deallocation Error : {:?}", error);}
    }
    memory::show_layout();

    // Run the Test functions across the entire crate
    #[cfg(test)]
    test_framework_entry_point();

    // continuous_keyboard_read();
}



