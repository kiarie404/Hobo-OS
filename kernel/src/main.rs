
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
use hobo_os::String as String;
use hobo_os::vec::Vec; // uses the alloc crate in the background


// defining the entry point function
// kinit returns the satp value .  
// this value gets used to update the satp register before executing kmain
#[no_mangle]
pub extern "C" fn kinit () {
    println!("I am in Machine mode... mad Chad");
    // Initialize stuff
        trap_handler::init_kernel_trap_handling(); // places a kernel trap frame in the mscratch register
        drivers::init_all_drivers();  // configure the drivers
        page_manager::init_memory();  // memory initialization... demarcates the physical memory into pages+descriptors
        byte_manager::init_kernel_byte_allocation(); // allocates kernel heap pages, demarcates the AllocList linked list on that heap 
    
    // Story Time
    println!("\n-------\n");
    println!("Neutral News : It is time for a story");
    println!("We are going to test whether the Byte allocator works for the Kernel Heap");
    println!(" To do that, we will have to do some things in machine mode ");
    println!("1. Allocate a byte");
    println!("2. Allocate a vector");
    println!("3. Allocate a string");
   


    print!("\n \n Quite the story... ready waste some time? (press any key) ---> ");
    stdin::read_line();
    println!("\n-------\n");

    // Test whether allocating a single byte works
    let allocated_byte_ptr = byte_manager::kzmalloc(1);
    let mut val:u8;
    unsafe {
        allocated_byte_ptr.write_volatile(10);
        val = allocated_byte_ptr.read_volatile();
        println!("Value has been read: {}", val);
    }

    println!("{}", val);
    if val == 10{  println!("Byte alloction works!!");  }
    

    // Test whether the rust global allocator 
    println!("\n-------\n");
    let allocated_String: String = String::from("some String");
    println!("{}", allocated_String);

    // Test whether we can use vectors
    println!("\n-------\n");
    let allocated_vector: Vec<i32> = [1,2,3].to_vec();
    println!("{:?}", allocated_vector );


    
    // [undone] : show unmapping

    println!("Switching to Supervisor mode...");
}

#[no_mangle]
pub extern "C" fn kmain() -> (){
    println!("\n-------\n");
    println!("We are in Supervisor Mode!");


    

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








