
#![no_std]  // we will no depend on Rust Standard Library and Libc
#![no_main] // we will define our own entry point sequence using the linker + Bootloader
#![feature(panic_info_message)]

use core::panic::PanicInfo;
use core::arch::asm;

// macros
use hobo_os::{print, println};
use hobo_os::drivers;
use hobo_os::interrupt_and_exception_handling as trap_handler;
use hobo_os::riscv;

// defining the entry point function
// kinit returns the satp value .  
// this value gets used to update the satp register before executing kmain
#[no_mangle]
pub extern "C" fn kinit () {
    println!("I am in Machine mode... mad Chad");
    trap_handler::init_kernel_trap_handling();
    drivers::init_all_drivers();

    println!("Switching to Supervisor mode...");
}

#[no_mangle]
pub extern "C" fn kmain() -> (){
    println!("Hello Superpowers, I am in supervisor mode!!!");

    println!("Test if stdin::read_line works...");
    println!("Enter a sentence that has less than 100 ascii characters...");

    // loop{}

    // An illegal instruction exception
    riscv::mcause_read();

    // endless loop keeps OS busy 
    // loop {}


    println!("Peace Out, I am going to shut down.... see you later.");
    return ();
}



// defining the function that will always get called after a panic
#[panic_handler]
fn panic_handler (panic_info: &PanicInfo) -> !{
    // make the current CPU_core sleep endlessly and wait for Interrupt
    let panic_location = panic_info.location().unwrap();
    let panic_message = panic_info.message().unwrap();
    // println!("Panic occured : in file : {}, line {}, with the message : {:?}",
    //                                              panic_location.file(), panic_location.line(), panic_message);
    loop {
       unsafe { asm!("wfi");  }
    }
}








