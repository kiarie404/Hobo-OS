
#![no_std]  // we will no depend on Rust Standard Library and Libc
#![no_main] // we will define our own entry point sequence using the linker + Bootloader
#![feature(panic_info_message)]

use core::panic::PanicInfo;
use core::arch::asm;


// mods used
use hobo_os::riscv;  // rust-wrapped RISCV instructions
use hobo_os::drivers; // import the UART, plic, Timer drivers
use hobo_os::{print, println, stdin}; // acts as std input/output functions
use hobo_os::interrupt_and_exception_handling as trap_handler; // indirectly lets you initialize the PLIC
use hobo_os::page_manager;
use hobo_os::byte_manager;
use hobo_os::sv39_mmu;
use hobo_os::map_kernel;
use hobo_os::interrupt_and_exception_handling::{TrapFrame, init_kernel_trap_handling};

use hobo_os::String as String;
use hobo_os::vec::Vec; // uses the alloc crate in the background

// import the BIG THREE
use hobo_os::{kernel_trap_frame, kernel_root_table_address_gl, kernel_satp_value_gl};

// defining the entry point function
// kinit returns the satp value .  
// this value gets used to update the satp register before executing kmain
#[no_mangle]
pub extern "C" fn kinit () {
    println!("I am in Machine mode... mad Chad");
    // Initialize stuff
        trap_handler::init_kernel_trap_handling(); // places a kernel trap frame static address in the mscratch register
        drivers::init_all_drivers();  // configure the drivers {PLIC, CLINT, UART}
        page_manager::init_memory();  // memory initialization... demarcates the physical memory into pages+descriptors
    
    // import and update the BIG THREE VARIABLES that will be used by the kernel while in supervisor mode
       let kernel_trapframe_ref: &mut TrapFrame = unsafe { &mut kernel_trap_frame };
       let kernel_satp_value_ref = unsafe { &mut kernel_satp_value_gl };
       let kernel_root_table_address_ref = unsafe { &mut kernel_root_table_address_gl};

       *kernel_root_table_address_ref = page_manager::alloc(1).unwrap();
       *kernel_satp_value_ref = 0usize | (8 << 60) | (*kernel_root_table_address_ref >> 12);

    // identity map the machine memory before switching to Supervisor mode
        map_kernel::identity_map_kernel(*kernel_root_table_address_ref);

    // initialize the kernel heap and make it byte-accessible
    // The Rust global allocator can only affect the kernel heap only
        byte_manager::init_kernel_byte_allocation(); // allocates kernel heap pages, demarcates the AllocList linked list on that heap 

    // update the Actual satp register
        // riscv::satp_write(*kernel_satp_value_ref as u64);
        // This produces a PageFault ERROR. Change the value of the SATP after you are inside kmain(). 

    // Done setting things up
        println!("\n-------\n");
        println!("Done setting things up ; Switching to Supervisor mode...");

}

#[no_mangle]
pub extern "C" fn kmain() -> (){
    println!("\n-------\n");
    println!("We are in Supervisor Mode!");

    // import and update the BIG THREE VARIABLES 
       let kernel_trapframe_ref: &mut TrapFrame = unsafe { &mut kernel_trap_frame };
       let kernel_satp_value_ref = unsafe { &mut kernel_satp_value_gl };
       let kernel_root_table_address_ref = unsafe { &mut kernel_root_table_address_gl};

    riscv::satp_write(*kernel_satp_value_ref as u64);

    // Show that the MMU is switched on --- Mode of SATP = 8
    println!("\n-------\n");
    let satp_value = riscv::satp_read();
    println!("As proof, here is the satp value; The SATP_MODE = 8");
    println!("SATP : {:064b}", satp_value);

    // Show that we can still access the entire RAM
    println!("\n-------\n");
    println!("Testing if we can still access every part of the RAM even if we are in supervisor... regardless of whether a page is allocated or not");
    let trans_result = sv39_mmu::translate(*kernel_root_table_address_ref as u64, 0x080005000);
    if trans_result.is_err() {  println!("\t Test Failed")}
    else {  println!("Test Passed"); }

    

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









