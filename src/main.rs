
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


// usage of accessible modules
use core::{arch::asm, panic::PanicInfo};
use core::fmt::Write; // enable the use of Write functions in this scope
use stdin::continuous_keyboard_read;

use crate::sv39_mmu::{map, show_mappings, unmap, translate};



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
    // page_manager::init_memory();
    // memory::show_layout();

    // let order = page_manager::check_descriptor_ordering();
    // if order == false { println!(">>> ordering of descriptors is messed up... ");}

    // let loc_1_address = page_manager::alloc(5).unwrap();
    // let loc_2_address = page_manager::alloc(10).unwrap();
    // page_manager::show_layout();


    // let order = page_manager::check_descriptor_ordering();
    // if order == false { println!(">>> ordering of descriptors is messed up... ");}

    // let dealloc_result = page_manager::dealloc(loc_2_address);
    // match  dealloc_result {
    //     Ok(x) => /*do nothing */{},
    //     Err(error) => {println!("Deallocation Error : {:?}", error);}
    // }
    // page_manager::show_layout();

    // Run the Test functions across the entire crate

    // test MMU 

    // page_manager::init_memory();
    // page_manager::check_descriptor_ordering();

    // println!("***** Allocating scace for root");
    // let root_table_address = page_manager::alloc(1).unwrap() as u64;
    // println!("****** Root table given address {:016x}",root_table_address );
    
    // println!("***** Allocating scace for pages we want to map");
    // let phy_1 = page_manager::alloc(1).unwrap() as u64;
    // let phy_2 = page_manager::alloc(1).unwrap() as u64;
    // let phy_3 = page_manager::alloc(1).unwrap() as u64;
    // let access_map = 0b110;

    // println!("****** Physical pages to be mapped:  {:016x}, {:016x}, {:016x}", phy_1, phy_2, phy_3);

    // map(0x200000, phy_1, access_map, root_table_address);
    // map(0x201000, phy_2, access_map, root_table_address);
    // map(0x202000, phy_3, access_map, root_table_address);
    // let order = page_manager::check_descriptor_ordering();
    // if order == false { println!(">>> ordering of descriptors is messed up... ");}


    // show_mappings(root_table_address);
    // page_manager::check_descriptor_ordering();

    // unmap(root_table_address);
    // page_manager::check_descriptor_ordering();
    // // page_manager::show_layout();

    // ---- testing Kernel Mapping -----// 
    page_manager::init_memory();
    let kernel_root_table_address = page_manager::alloc(1).unwrap();

    // identity mapping
    map_kernel::identity_map_kernel(kernel_root_table_address);

    // proof by showing the mappings
    sv39_mmu::show_mappings(kernel_root_table_address as u64);

    // mode : sv39
    // ASID : 0
    // address : kernel_root_table_address
    let satp_value: usize = 0usize | (8 << 60) | (kernel_root_table_address >> 12);
    println!("Satp Value: {}", satp_value);
    // return satp_value;

    // #[cfg(test)]
    // test_framework_entry_point();

    // continuous_keyboard_read();
}


#[no_mangle]
pub extern "C" fn kmain(){
    println!("Hello world, I am in supervisor mode!!!");
}



