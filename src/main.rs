
#![no_std]  // we will no depend on Rust Standard Library and Libc
#![no_main] // we will define our own entry point sequence using the linker + Bootloader
#![feature(panic_info_message)]


// mod utilities;
mod drivers;
mod asm;
mod stdout;
mod stdin;
mod memory;


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
    continuous_keyboard_read();
}

