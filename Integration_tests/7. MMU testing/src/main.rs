
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
use hobo_os::sv39_mmu;


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
    
    // Story Time
    println!("\n-------\n");
    println!("Neutral News : It is time for a story");
    println!("We are going to test whether the sv39 mmu works");
    println!(" To do that, we will have to do some things in machine mode ... and some other things in supervisor mode");
    println!("1. Allocate space for a root table");
    println!("2. Allocate 3 pages that we will eventually map");
    println!("3. show the unhappy paths of mapping");
    println!("4. Map those 3 pages without any errors");
    println!("5. Show the page table mappings, they should be correct");
    println!("6. Show the happy path of the translation process");
    println!("7. Show the unhappy paths of translating");
    println!("8. Show the unhappy paths of unmapping ");
    println!("9. Show the happy path of unmapping");


    print!("\n \n Quite the story... ready waste some time (press any key) ---> ");
    stdin::read_line();
    println!("\n-------\n");

    // initialize variables
    let root_table_address = page_manager::alloc(1).unwrap() as u64;
    let first_physical_address = page_manager::alloc(3).unwrap() as u64;
    let second_physical_address = first_physical_address + 4096;
    let third_physical_address = second_physical_address + 4096;
    let first_virtual_page_address = 0x0200_1000;
    let second_virtual_page_address = 0x0200_2000;
    let third_virtual_page_address = 0x0200_3000;
    let access_map = 2u64;

    // display the FACTS
    println!("\n-------\n");
    println!("Here are the facts before we begin the tests");
    println!("Root Table address : 0x{:08x}", root_table_address);
    println!("first_physical_address : 0x{:08x}", first_physical_address);
    println!("second_physical_adress : 0x{:08x}", second_physical_address);
    println!("third_physical_adress : 0x{:08x}", third_physical_address);
    println!("first_virtual_page_address : 0x{:08x}", first_virtual_page_address);
    println!("second_virtual_page_address : 0x{:08x}", second_virtual_page_address);
    println!("third_virtual_page_address : 0x{:08x}", third_virtual_page_address);

    // show unhappy paths of mapping
        println!("\n-------\n");
        println!("Mapping a Non_page physical address should FAIL. For Example : 0x80097067");
        let map_result = sv39_mmu::map(first_virtual_page_address, 0x80097067, access_map, root_table_address);
        match map_result {
            Ok(()) => println!("Test Failed : mapping a non_page address should fail"),
            Err(map_err) => println!("{:?}", map_err)
        }

        println!("\n-------\n");
        println!("Mapping a Non_page virual address should FAIL. For Example : 0x80097067");
        let map_result = sv39_mmu::map(0x80097067, 0x80097067, access_map, root_table_address);
        match map_result {
            Ok(()) => println!("Test Failed : mapping a non_page address should fail"),
            Err(map_err) => println!("{:?}", map_err)
        }


        println!("\n-------\n");
        println!("Mapping a Wrong access mask should FAIL. For Example : 0b0111111 should fail");
        let map_result = sv39_mmu::map(first_virtual_page_address, first_physical_address, 0b0111111, root_table_address);
        match map_result {
            Ok(()) => println!("Test Failed : mapping a non_page address should fail"),
            Err(map_err) => println!("{:?}", map_err)
        }
    
    // Show the Happy Path of Mapping
        println!("\n-------\n");
        println!("Mapping the right values should PASS...");
        let _map_result = sv39_mmu::map(first_virtual_page_address, first_physical_address, access_map, root_table_address).unwrap();
        let _map_result_2 = sv39_mmu::map(second_virtual_page_address, second_physical_address, access_map, root_table_address).unwrap();
        let _map_result = sv39_mmu::map(third_virtual_page_address, third_physical_address, access_map, root_table_address).unwrap();

    
    // Show Mappings
        println!("\n-------\n");
        println!("Showing the simplified page Table...\n");
        sv39_mmu::show_mappings(root_table_address);
        
    // Showing that translation works
        println!("\n-------\n");
        println!("Translating {:08x} should yield {:08x} as seen in the Page Table...", first_virtual_page_address, first_physical_address);
        let translation_result = sv39_mmu::translate(root_table_address, first_virtual_page_address);
        match translation_result {
            Ok(physical_address) => {
                if physical_address == first_physical_address{  println!("\t Translation works!!");  }
                else {
                    println!("\t Trnslation does not work, it gives wrong translations. Instead of yielding {:08x}, it yielded {:08x}", physical_address, first_physical_address);
                }
            }
            Err(trans_err) => println!("{:?}", trans_err)
        }
    
    // Showing some translation errors
        println!("\n-------\n");
        println!("Translating a virtual address that has not been allocated should give an error: for example {:08x} should FAIL", third_virtual_page_address + 4096);
        let translation_result = sv39_mmu::translate(root_table_address, third_virtual_page_address + 4096);
        match translation_result {
            Ok(physical_address) => println!("Test Failed: the translation yielded {:08x}", physical_address),
            Err(trans_err) => println!("Test Passed, it returned a translation error : {:?}", trans_err)
        }
    
    // [undone] : show all translation errors

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









