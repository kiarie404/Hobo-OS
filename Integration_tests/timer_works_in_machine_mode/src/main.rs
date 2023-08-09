
#![no_std]  // we will no depend on Rust Standard Library and Libc
#![no_main] // we will define our own entry point sequence using the linker + Bootloader
#![feature(panic_info_message)]

use core::panic::PanicInfo;
use core::arch::asm;

// macros
use hobo_os::{print, println, TrapFrame};
// libraries
use hobo_os::drivers;
use hobo_os::stdout;
use hobo_os::Timer;
use hobo_os::riscv;

// defining the entry point function
// kinit returns the satp value .  
// this value gets used to update the satp register before executing kmain
#[no_mangle]
pub extern "C" fn kinit () {
    println!("I am in Machine mode...");

    // store kernel trapframe into the mscratch register
    let kernel_trapframe_ref = unsafe{&hobo_os::kernel_trap_frame};
    let kernel_trapframe_ptr = kernel_trapframe_ref as *const TrapFrame;
    let kernel_trapframe_address = kernel_trapframe_ptr as u64;
    riscv::mscratch_write(kernel_trapframe_address);


}

#[no_mangle]
pub extern "C" fn kmain() -> (){
    // Show that we are in supervisor mode
    println!("Hello world, I am in supervisor mode!!!");
    
    Timer::mtimecmp_write(Timer::mtime_read() + 1000);
    
    println!("hahaha, I am going to shut down.... see you later.");
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








