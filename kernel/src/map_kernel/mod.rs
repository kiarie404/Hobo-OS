//! This module identity maps the Kernel addresses.  
//! It maps the entire RAM and specific MMIO regions
//! 
//! 
//! The MMIO regions covered include : the UART, the CLINT and the PLIC.    
//! Now the kernel can access all relevant memory regions while using the virtual paging system

use crate::sv39_mmu::{map, show_mappings};
use crate::page_manager::alloc;
use crate::{print, println};

// defining constants and relevant global variables
const PAGE_SIZE: usize = 4096;

// memory addresses that the KERNEL will always need :
// THis variables have been imported through the crate/asm/mem_export.s file
// ------------  The RAM sections ------------------------ //
extern "C"{
	static TEXT_START: usize;
	static TEXT_END: usize;
	static DATA_START: usize;
	static DATA_END: usize;
	static RODATA_START: usize;
	static RODATA_END: usize;
	static BSS_START: usize;
	static BSS_END: usize;
	static KERNEL_STACK_START: usize;
	static KERNEL_STACK_END: usize;
	static HEAP_START: usize;
	static HEAP_END: usize; 
}


// ------- THe MMIO Sections ------------------------------ //
// https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c has the Memory Layout of the adresses below.    
// The Specific registers like MTIME and MTIMECMP addresses are got by adding offsets to the MMIO BAse Address.  
// The Offsets are defined at : https://github.com/qemu/qemu/blob/ccdd31267678db9d80578b5f80bbe94141609ef4/include/hw/intc/riscv_aclint.h#L72
// Here is the offset definition :
// enum {
//     RISCV_ACLINT_DEFAULT_MTIMECMP      = 0x0,
//     RISCV_ACLINT_DEFAULT_MTIME         = 0x7ff8,
//     RISCV_ACLINT_DEFAULT_MTIMER_SIZE   = 0x8000,
//     RISCV_ACLINT_DEFAULT_TIMEBASE_FREQ = 10000000,
//     RISCV_ACLINT_MAX_HARTS             = 4095,
//     RISCV_ACLINT_SWI_SIZE              = 0x4000
// };
static UART_ADDRESS_START : usize = 0x1000_0000;
static UART_ADDRESS_END : usize = 0x1000_0000;

// The Rest of the addresses have been adopted from the OS Site tutorial
// How the exact addresses were calculated I DO NOT KNOW
// static CLINT_ADDRESS : usize = 0x0200_0000;
// static MTIMECMP_ADDRESS : usize = 0x0200_b000;
// static MTIME_ADDRESS_START : usize = 0x0200_c000;
// static UART_ADDRESS : usize = 
// static UART_ADDRESS : usize = 


// THis function assumes that the memory has already been initialized
// It also assumes that the KERNEL_ROOT_TABLE_ADDRESS is currently a Mutable Static Address
pub fn identity_map_kernel(root_table_address: usize){
    map_the_ram_sections(root_table_address);
    map_the_mmio_sections(root_table_address);
}

/// identity_map_many_pages takes in 3 physical addresses  
/// 1. Start Address : This is the address of the first Page in a range of contiguous pages that the kenel may need.
/// 2. The End Address : THis is the addresss if the last Page in that range of contiguous pages
/// 3. The root table address : This is the address of the root table. Where the translation table of the kernel process will be
/// This function identity maps a group of pages.
fn identity_map_many_pages( start_address: usize, end_address: usize, root_table_address: usize, access_map: u64) {
    // align the addresses : we do not care about aligning th start addresses because we aligned them in the Linker script
    let aligned_start_address = align(start_address, 12);
    let aligned_end_address = align(end_address, 12);

    // loop through the range of addresses in a page-wise manner:
    let mut page_address = aligned_start_address as u64;
    while page_address < aligned_end_address as u64{
        map(page_address, page_address, access_map, root_table_address as u64).expect("Unable to Identity Map pages");
        page_address += PAGE_SIZE as u64;
    }

}


// aligns memory to the specified order
// Before we give any address to the identity_map_many_pages(), we need to make sure the addresses are aligned to 4096
fn align (val: usize, order: usize) -> usize{
    let addition_mask : usize = 1 << order;    // eg if we need to find things in order of 2 ie 2^2, the number will always have 2 zeroes at the LSB
    let over_board : usize = val + addition_mask; // so when we get a non_mutiple number, we make sure it passes the next multiple.  
    let cut_mask : usize = !0 << order; // a mask that will be used to make the last 2 bits become zeroes
    let result: usize = over_board & cut_mask; // replace the last 2 bits with zeroes
    return result;
}

fn map_the_ram_sections(root_table_address: usize){
    unsafe {
        // map the text section
        let mut access_map: u64 = 10u64; // Read-Execute access
        identity_map_many_pages(TEXT_START, TEXT_END, root_table_address, access_map);

        // map the rodata section
        access_map = 10u64; // Read-Execute access
        identity_map_many_pages(RODATA_START, RODATA_END, root_table_address, access_map);

        // map the data section
        access_map = 6u64; // Read-Write access
        identity_map_many_pages(DATA_START, DATA_END, root_table_address, access_map);

        // map the bss section
        access_map = 6u64; // Read-Write access
        identity_map_many_pages(BSS_START, BSS_END, root_table_address, access_map);

        // map the Kernel stack
        access_map = 6u64; // Read-Write access
        identity_map_many_pages(KERNEL_STACK_END, KERNEL_STACK_END, root_table_address, access_map);

        // map the entire Heap
        access_map = 6u64; // Read-Write access
        identity_map_many_pages(HEAP_START, HEAP_END, root_table_address, access_map); 
    }
}

fn map_the_mmio_sections(root_table_address: usize){
    unsafe{
        // map the UART
        let mut access_map : u64 = 6u64; // Read-Write access
        map(0x1000_0000, 0x1000_0000, access_map, root_table_address as u64).unwrap();

        // map the PLIC
        identity_map_many_pages(0x0c00_0000, 0x0c00_2000, root_table_address, access_map);
        identity_map_many_pages(0x0c20_0000, 0x0c20_8000, root_table_address, access_map);

        // map the CLINT
            // MSIP
            map(0x0200_0000, 0x0200_0000, access_map, root_table_address as u64).unwrap();

            // MTIME
            map(0x0200_c000, 0x0200_c000, access_map, root_table_address as u64).unwrap();

            // MTIMECMP
            map(0x0200_b000, 0x0200_b000, access_map, root_table_address as u64).unwrap();

    }
}



