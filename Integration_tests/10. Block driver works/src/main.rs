
#![no_std]  // we will no depend on Rust Standard Library and Libc
#![no_main] // we will define our own entry point sequence using the linker + Bootloader
#![feature(panic_info_message)]

use core::panic::PanicInfo;
use core::arch::asm;


// mods used
use hobo_os::riscv;  // rust-wrapped RISCV instructions
use hobo_os::{drivers, drivers::virtio_block as block}; // import the UART, plic, Timer drivers
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
        drivers::init_all_hardwired_drivers();  // configure the drivers {PLIC, CLINT, UART}. This does NOT include things like HardDisks which are attached instead of hardwired
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

    // initialize the Hard-Disk via the Block Driver
        drivers::probe_and_initialize_virtio_devices();

        let mut buffer_ptr = byte_manager::kzmalloc(1024); // Where we will store data read from the Hard Disk
        block::read(8, buffer_ptr, 512, 1024); // hdd is attached at Bus 8. The reads data as 512byte blocks. We skip the Superblok(1024)'

        let mut curr_memory_index = 0; // This is the index of memory that we are currently accessing in the hard-Disk


    // Begin the Test
    println!("\n---------------------\n");
    println!("This is a test for the integration of the Block device into the kernel while in machine mode");
    println!("\n---------------------\n");

    println!("The Test will validate that the following actions are possible: ");
    println!("1. The block device can be written to");
    println!("2. The block device can be read from");
    println!("3. The block device stores data persistently");

    println!("\n---------Test 1 : Writing to the block device------------\n");
    println!("\t We will try to write the word 'sword' into the memory address that comes 2 pages after the first page");
    let size= 1024;  // the size of the kind of data we want to mess with
    let write_buffer = byte_manager::kzmalloc(1024); // Data is read as 512-byte sectors , so out buffer is atleast 512 bytes
    let dev = 8;   // the Block device is stores at Virtio Bus 8
    let offset = 0x0000_2000; // let the offset be 2 Pages from the start of the block device

        // Store the word "Sword" in the 1024-byte buffer
        // Make sure the writes and Reads are ALL VOLATILE
        unsafe{
            write_buffer.add(0).write_volatile('S' as u8);
            write_buffer.add(1).write_volatile('W' as u8);
            write_buffer.add(2).write_volatile('0' as u8);
            write_buffer.add(3).write_volatile('R' as u8);
            write_buffer.add(4).write_volatile('D' as u8);
        }

    // Comment out THIS LINE in order to run test 3. Commenting this out ensures that test 2 reads persistent that was written in the previous kernel run 
    // block::write(dev, write_buffer, size, offset);  


    println!("\n---------Test 2 : Reading from the block device------------\n");
    println!("\t We will try to read the word 'sword' from the memory address that comes 2 pages after the first page");  
    // And we will store it in a different buffer, just to make  sure we are not using thr previous write_buffer that already contains the right value 
    let read_buffer_ptr = byte_manager::kzmalloc(1024);
    let read_buffer_ref = unsafe { & *read_buffer_ptr};
    block::read(dev, read_buffer_ptr, size, offset);

    println!("\t Printing Out the first 5 letters of the read_buffer : ");
    for count in 0..5{
        let ptr = unsafe{read_buffer_ptr.add(count).read_volatile()};
        // let value_ref = unsafe{& *ptr};
        print!("{}", ptr as char);
    }

    println!("\n---------Test 3 : Testing if the Memory is persistent ------------\n");
    println!("\t To perform this test...");  
    println!("\t\t 1. Run Test 1 so that you can write the word SWORD into memory ");
    println!("\t\t 2. Comment out test 1 and uncomment test 2 and recompile the code. Comment out line 95 to be precise");   
    println!("\t\t 3. Test 2 should run smoothly even though the write to memory is no longer happening"); 





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

    // Testing The Block Driver in kmain()

    

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









