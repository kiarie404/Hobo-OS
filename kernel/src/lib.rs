
#![no_std]  // we will no depend on Rust Standard Library and Libc
#![no_main] // we will define our own entry point sequence using the linker + Bootloader
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)] // enable the use of a custom test framework
#![test_runner(crate::test_framework::test_runner)]
#![reexport_test_harness_main = "test_framework_entry_point"] // give the entrypoint a custom name, and add it to the program flow
#![feature(error_in_core)] // so as to use the Error trait 
#![feature(alloc_error_handler)] // alloc_error_handler, is used to define a custom error handler for out-of-memory situations in programs...
                                // that use the Rust standard library's allocator (std::alloc). 
                                // This feature allows developers to have more control over how their program behaves when memory allocation fails due to running out of available memory
// External crates
extern crate alloc;
extern crate core;

// mod utilities;
pub mod drivers;
pub mod asm;
pub mod stdout;
pub mod stdin;
pub mod page_manager;
pub mod sv39_mmu;
pub mod test_framework; 
pub mod map_kernel;
pub mod riscv;
pub mod interrupt_and_exception_handling;
pub mod byte_manager;



// usage of accessible modules
use core::{arch::asm, panic::PanicInfo};
use core::fmt::Write; // enable the use of Write functions in this scope
pub use alloc::{string::String, vec};

pub use crate::interrupt_and_exception_handling::TrapFrame;
pub use crate::sv39_mmu::{map, show_mappings, unmap, translate};
pub use crate::drivers::timer::Timer;

pub static mut kernel_satp_value_gl: usize = 0;
pub static mut kernel_root_table_address_gl : usize = 0;
pub static mut kernel_trap_frame : TrapFrame = TrapFrame::zero();





// // defining the entry point function
// // kinit returns the satp value .  
// // this value gets used to update the satp register before executing kmain
// #[no_mangle]
// pub extern "C" fn kinit () {
//     println!("I am in Machine mode...");
//     // // You can access both mstatus and sstatus because you are in machine mode
//     // let mstatus_value = riscv::mstatus_read();
//     // let sstatus_value = riscv::sstatus_read();
//     // println!("mstatus : {:b}", mstatus_value);
//     // println!("sstatus : {:b}", sstatus_value);


//     // // initialize memory
//     // page_manager::init_memory();

//     // // get kernel_root_table
//     // let kernel_root_table_address = page_manager::alloc(1).unwrap();

//     // // identity map the kernel address space
//     // map_kernel::identity_map_kernel(kernel_root_table_address);

//     // // show that the mappings are okay
//     // sv39_mmu::show_mappings(kernel_root_table_address as u64);

//     // // show that translation works
//     // let virt_uart_address = 0x1000_0000;
//     // let physical_uart_address = sv39_mmu::translate(kernel_root_table_address as u64, virt_uart_address).unwrap();
//     // println!("Uart Address : {:016x}", physical_uart_address);

//     // // define the address of the TrapFrame
//     // // let mut trap_frame = interrupt_and_exception_handling::TrapFrame::zero();
//     // let trap_frame_ref = unsafe { &mut kernel_trap_frame};
//     // let trap_frame_ptr = trap_frame_ref as *mut TrapFrame;
//     // let trap_frame_address = trap_frame_ptr as usize;
//     // // // update the mscratch register with the address of the trapframe
//     // unsafe {riscv::mscratch_write(trap_frame_ptr as u64);}
//     // unsafe {println!("address while in kinit : {}", trap_frame_ptr as u64)};


//     // // update the kernel satp value and kernel root table address global
//     // unsafe{
//     //     kernel_satp_value_gl = 0usize | (8 << 60) | (kernel_root_table_address >> 12);
//     //     kernel_root_table_address_gl =  kernel_root_table_address;
//     // }

    
//     // let x = 12;
//     // let y = 20;
//     // unsafe {asm!("add t5, {}, {}", in(reg)x, in(reg)y )};
//     // let q : usize;
//     // unsafe {asm!("add {}, t5, zero", out(reg)q)};
//     // println!("q : {}", q);

    

//     // // println!("{:?}", dup_ref);
//     // // unsafe {println!("address while in kinit : {}", trap_frame_ptr as u64)};
    
//     // Test for a timer interrupt
//     // Timer::mtimecmp_write(Timer::mtime_read() + 10_000_000); // interrupt will happen after a second
//     // println!("mtimecmp: {}", Timer::mtimecmp_read());
//     // println!("mtime: {}", Timer::mtime_read());
//     // println!("{}", riscv::mscratch_read());


    

// }




// #[no_mangle]
// pub extern "C" fn kmain() -> (){
//     // Show that we are in supervisor mode
//         println!("Hello world, I am in supervisor mode!!!");
//         // let mstatus_value = riscv::mstatus_read();    // WILL NOT WORK BECAUSE WE ARE IN SUPERVISOR MODE
//     //     let sstatus_value = riscv::sstatus_read();
//     //     // println!("mstatus : {:b}", mstatus_value);   // WILL NOT WORK 
//     //     println!("sstatus : {:b}", sstatus_value);

//     // // Access the kernel_root_table_address and satp value
//     // let kernel_root_table_address = unsafe {kernel_root_table_address_gl};
//     // let kernel_satp_value = unsafe {kernel_satp_value_gl};

//     // // Show that we can still translate addresses 
//     //     let virt_uart_address = 0x1000_0000;
//     //     let physical_uart_address = sv39_mmu::translate(kernel_root_table_address as u64, virt_uart_address).unwrap();
//     //     println!("Uart Address : {:016x}", physical_uart_address);


//     // let q : usize;
//     // unsafe {asm!("add {}, t5, zero", out(reg)q)};
//     // println!("q : {}", q);

//     // // define the address of the TrapFrame
//     // // let mut trap_frame = interrupt_and_exception_handling::TrapFrame::zero();
//     // let trap_frame_ref = unsafe { &mut kernel_trap_frame};
//     // let trap_frame_ptr = trap_frame_ref as *mut TrapFrame;
//     // let trap_frame_address = trap_frame_ptr as usize;

//     // let trap_frame_address = riscv::mscratch_read();
//     // println!("See the error did not stop the program flow");

//     // Test for a timer interrupt
//     Timer::mtimecmp_write(Timer::mtime_read() + 10_00); // interrupt will happen after a second

//     // test for an environment call exception
//     unsafe{ asm!("ecall")};

    
//     println!("hahaha, I am going to shut down.... see you later.");
//     return ();
// }

// aligns memory to the specified order
// Before we give any address to the identity_map_many_pages(), we need to make sure the addresses are aligned to 4096
fn align (val: usize, order: usize) -> usize{
    let addition_mask : usize = 1 << order;    // eg if we need to find things in order of 2 ie 2^2, the number will always have 2 zeroes at the LSB
    let over_board : usize = val + addition_mask; // so when we get a non_mutiple number, we make sure it passes the next multiple.  
    let cut_mask : usize = !0 << order; // a mask that will be used to make the last 2 bits become zeroes
    let result: usize = over_board & cut_mask; // replace the last 2 bits with zeroes
    return result;
}




