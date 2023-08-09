
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


// defining the entry point function
// kinit returns the satp value .  
// this value gets used to update the satp register before executing kmain
#[no_mangle]
pub extern "C" fn kinit () {
    println!("I am in Machine mode... mad Chad");
    // Initialize stuff
        trap_handler::init_kernel_trap_handling();
        drivers::init_all_drivers();
        page_manager::init_memory();  // memory initialization
    
    // Show layout after initialization 
    println!("\n-------\n");
    print!("Enter any key to show layout after initialization ---> ");
    stdin::read_line();
    println!("\n-------\n");
    page_manager::show_layout(); // show how to reduce time of parsing descriptors {algo or negligence}

    // Show allocation in machine mode : working    
    println!("\n-------\n");
    print!("Enter any key to show allocation (Set pages statically... for now)---> ");
    stdin::read_line();
    println!("\n-------\n");
    let alloc_result = page_manager::alloc(5);
    let mut allocation_start_adress : usize = 0;
    match alloc_result{
        Ok(address) => allocation_start_adress = address,
        Err(alloc_error) => println!("{:?}", alloc_error)
    }

    // Show memory map after allocation
        page_manager::show_layout(); // heapLayout stats
        
    // Show Deallocation in machine mode    
    println!("\n-------\n");
    print!("Enter any key to show De-allocation (the previously allocated pages)---> ");
    stdin::read_line();
    println!("\n-------\n");
    let dealloc_result = page_manager::dealloc(allocation_start_adress);
    match dealloc_result{
        Ok(address) => {/* do nothing */},
        Err(dealloc_error) => println!("{:?}", dealloc_error)
    }

    page_manager::show_layout();

    //  ---------   ERRORS  in allocation ------------------------------// 
    // Show allocation in machine mode : problematic    
    println!("\n-------\n");
    print!("Enter any key to show allocation of an insane amount of pages (1000000000)---> ");
    stdin::read_line();
    println!("\n-------\n");
    let alloc_result = page_manager::alloc(1000000000);
    match alloc_result{
        Ok(address) => allocation_start_adress = address,
        Err(alloc_error) => println!("{:?}", alloc_error)
    }

    // Show allocation in machine mode : problematic    
    println!("\n-------\n");
    print!("Enter any key to show allocation of an insane amount of pages (0)---> ");
    stdin::read_line();
    println!("\n-------\n");
    let alloc_result = page_manager::alloc(0);
    match alloc_result{
        Ok(address) => allocation_start_adress = address,
        Err(alloc_error) => println!("{:?}", alloc_error)
    }



    //  ------------------- ERRORS in Deallocation ----------------------- //   
    println!("\n-------\n");
    print!("Enter any key to show deallocation of an address that is not part of the heap (0x0000) ---> ");
    stdin::read_line();
    println!("\n-------\n");
    let dealloc_result = page_manager::dealloc(0);
    match dealloc_result{
        Ok(address) => {/* do nothing */},
        Err(dealloc_error) => println!("{:?}", dealloc_error)
    }
    


    println!("\n-------\n");
    print!("Enter any key to show deallocation of an address that is not a valid page address (valid address + any_number_that_is_not_4096) ---> ");
    stdin::read_line();
    println!("\n-------\n");
    let dealloc_result = page_manager::dealloc(allocation_start_adress + 4);
    match dealloc_result{
        Ok(address) => {/* do nothing */},
        Err(dealloc_error) => println!("{:?}", dealloc_error)
    }

    println!("\n-------\n");
    print!("Enter any key to show deallocation of an address that has already been deallocated ---> ");
    stdin::read_line();
    println!("\n-------\n");
    let dealloc_result = page_manager::dealloc(allocation_start_adress );
    match dealloc_result{
        Ok(address) => {/* do nothing */},
        Err(dealloc_error) => println!("{:?}", dealloc_error)
    }


    println!("Switching to Supervisor mode...");
}

#[no_mangle]
pub extern "C" fn kmain() -> (){
    println!("Hello Superpowers, I am in supervisor mode!!!");

    

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









