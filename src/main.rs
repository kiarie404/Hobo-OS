
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
mod page_manager;
mod sv39_mmu;
mod test_framework; 
mod map_kernel;
mod riscv;


// usage of accessible modules
use core::{arch::asm, panic::PanicInfo};
use core::fmt::Write; // enable the use of Write functions in this scope
use stdin::continuous_keyboard_read;

use crate::sv39_mmu::{map, show_mappings, unmap, translate};

static mut kernel_satp_value_gl: usize = 0;
static mut kernel_root_table_address_gl : usize = 0;


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
// kinit returns the satp value .  
// this value gets used to update the satp register before executing kmain
#[no_mangle]
pub extern "C" fn kinit () {
    println!("I am in Machine mode...");
    // You can access both mstatus and sstatus because you are in machine mode
    let mstatus_value = riscv::mstatus_read();
    let sstatus_value = riscv::sstatus_read();
    println!("mstatus : {:b}", mstatus_value);
    println!("sstatus : {:b}", sstatus_value);


    // initialize memory
    page_manager::init_memory();

    // get kernel_root_table
    let kernel_root_table_address = page_manager::alloc(1).unwrap();

    // identity map the kernel address space
    map_kernel::identity_map_kernel(kernel_root_table_address);

    // show that the mappings are okay
    sv39_mmu::show_mappings(kernel_root_table_address as u64);

    // show that translation works
    let virt_uart_address = 0x1000_0000;
    let physical_uart_address = sv39_mmu::translate(kernel_root_table_address as u64, virt_uart_address).unwrap();
    println!("Uart Address : {:016x}", physical_uart_address);

    // update the kernel satp value and kernel root table address global
    unsafe{
        kernel_satp_value_gl = 0usize | (8 << 60) | (kernel_root_table_address >> 12);
        kernel_root_table_address_gl =  kernel_root_table_address;
    }


}


#[no_mangle]
pub extern "C" fn kmain(){
    // Show that we are in supervisor mode
        println!("Hello world, I am in supervisor mode!!!");
        // let mstatus_value = riscv::mstatus_read();    // WILL NOT WORK BECAUSE WE ARE IN SUPERVISOR MODE
        let sstatus_value = riscv::sstatus_read();
        // println!("mstatus : {:b}", mstatus_value);   // WILL NOT WORK 
        println!("sstatus : {:b}", sstatus_value);

    // Access the kernel_root_table_address and satp value
    let kernel_root_table_address = unsafe {kernel_root_table_address_gl};
    let kernel_satp_value = unsafe {kernel_satp_value_gl};

    // Show that we can still translate addresses 
        let virt_uart_address = 0x1000_0000;
        let physical_uart_address = sv39_mmu::translate(kernel_root_table_address as u64, virt_uart_address).unwrap();
        println!("Uart Address : {:016x}", physical_uart_address);


}



