//! This module abstracts the Riscv SV39 MMU. It provides mapping, unmapping and translation functions. You can additionally inspect the tables

mod mmu_abstractions;
mod errors;
mod tests;

pub use mmu_abstractions::{Table, TableEntry};
use errors::MappingError;
use crate::page_manager;
use crate::{print, println};

/// The Map Function 
/// Each Process gets its own Translation Tables ie. Root_Table, Mid_Table, Leaf_Table  
/// The Map Function populates the Translation tables for that process   
/// Whenever a process gets a new page allocated to it, the new Virtual page address and the corresponding new physical Address need to 
/// get stored to the translation Tables of the specific process.  
///  
/// The Map Function takes the Virtual address and the Physical address and puts them in the Translation Table (populates the table)
/// Inputs for the function :   
///       1. Root Physical address extracted from SATP  
///       2. Valid Virtual address to a Page    
///          1. within the 39 bit range 
///          2. divisible by 4096... ie it is a valid page address  
///       3. Valid Physical address to a page   
///          1. Extracting from the Page allocator will guaratee validity   
///          2. within the 56 bit range 
///          3. divisible by 4096   
///       4. Valid access permissions {at least one specification to be provided}  
/// 
///  
/// 
/// [remove]
///  Warning, to you future reader, this map function assumes that the physical addresses being passed to it have been allocated validly.
/// But when Identity mapping the Kernel, you may choose not pass to it allocated addresses.  
/// Unmapping deallocates all the physical pages referenced by the page tables. So if you passed unallocated pages to the map function,
///  deallocation will be erroneous.  
/// ===> Keep that in mind. 
/// 
/// THis model of unmapping was chosen based on the design that at no point will physical addresses be shared by processes.... 
/// unless the kernel lends both processes its own space by syscalls

//    2. Errors : incorrect access specifications 
pub fn map(virt_address: u64, physical_address: u64, access_map: u64, root_table_address: u64) -> Result<(), errors::MappingError>{
    // validate all function inputs
        if validate_virtual_address(virt_address) == false {  
            return Err(errors::MAPPING_ERROR_InvalidVirtualAddress);
        }

        else if validate_physical_address(physical_address) == false {  
            return Err(errors::MAPPING_ERROR_InvalidPhysicalAddress);
        }

        else if validate_access_map(access_map) == false {  
            return Err(errors::MAPPING_ERROR_InvalidAccessMap);
        }

        else{
            // Extract the parts of the virtual address
            let root_table_index = (virt_address >> 30) &  0b111111111;
            let mid_table_index = (virt_address >> 21) &  0b111111111;
            let leaf_table_index = (virt_address >> 12) &  0b111111111;
            let page_offset = virt_address & 0b111111111111;

            // mutably access root entry
            let root_table_ptr = root_table_address as *mut Table;
            let root_table_ref = unsafe {&mut *root_table_ptr};
            let root_table_entry = &mut root_table_ref.content[root_table_index as usize];
            
            // We need to traverse the Page tables up to the leaf table.    
            // The virtual address will define our traversal path. We will store the physical address on an entry of the leaf table

            // Get the address of the mid_page_table    
            // The address is stored in the entry of the root table
                let mut mid_table_address : u64;
                // check if entry points to a valid mid_page_table in the first place  
                if root_table_entry.check_if_valid() == true {
                    mid_table_address = root_table_entry.get_address();
                }
                else { // make that table entry to point at a valid Page Table
                    let new_table = page_manager::alloc(1).expect("unable to allocate a Page for the Mid Table") as u64;
                    // println!("***** Mid table was given the address : {:016x}",new_table );
                    root_table_entry.set_address(new_table);
                    root_table_entry.set_as_valid();
                    mid_table_address = new_table;
                }
                // get a mutable reference to the mid table entry
                let mid_table_ptr = mid_table_address as *mut Table;
                let mid_table_ref = unsafe {&mut *mid_table_ptr};
                let mid_table_entry = &mut mid_table_ref.content[mid_table_index as usize];

            // Get the address of the leaf_page_table
                let mut leaf_table_address : u64;
                
                // check if mid table entry points to a valid leaf_page_table in the first place 
                if mid_table_entry.check_if_valid() == true {
                    leaf_table_address = mid_table_entry.get_address();
                }
                else { // make that table entry to point at a valid Page Table ie. Leaf Page Table
                    let new_table = page_manager::alloc(1).expect("unable to allocate a Page for the Leaf Table") as u64;
                    // println!("***** Leaf table was given the address : {:016x}",new_table );
                    mid_table_entry.set_address(new_table);
                    mid_table_entry.set_as_valid();
                    leaf_table_address = new_table;
                }

            // Get a mutable reference to the Leaf table entry
                let leaf_table_ptr = leaf_table_address as *mut Table;
                let leaf_table_ref = unsafe {&mut *leaf_table_ptr};
                let leaf_table_entry = &mut leaf_table_ref.content[leaf_table_index as usize];
                
            // Set the leaf entry to point to the physical Page address
                leaf_table_entry.set_address(physical_address);
                leaf_table_entry.set_as_valid();
                leaf_table_entry.add_access_mask(access_map);
            
            return Ok(());
        }
}



/// This function returns the physical address that corresponds to the input virtual address     
/// If the virtual address cannot be transalted, a None value is returned   
/// If the virtual address can be translated, the physical address is returned in a Some() wrapper   
/// A virtual address may not get translated because :  
///     1. The virtual address is beyond the 39 bit value range     
///     2. The Virtual address is referencing an address that has not yet been allocated to the process that is using that address

pub fn translate(root_table_address: u64, virt_address: u64) -> Result<u64, errors::TranslationError>{
    // validate the virtual address
        // check if address is out of range
        if virt_address > 2u64.pow(39) { return Err(errors::TRANS_ERROR_NonRangeVirtualAddress); }
        else { /* continue with the function... */}
    
    // after validation, we move through the traslation table till we hit a dead End or find a leat Page Table Entry
        // extract the table indexes and the offset from the virtual address
            let root_table_index = (virt_address >> 30) &  0b111111111;
            let mid_table_index = (virt_address >> 21) &  0b111111111;
            let leaf_table_index = (virt_address >> 12) &  0b111111111;
            let page_offset = virt_address & 0b111111111111;

        // loop through the translation table
            // get entry under the root table
            let root_table_ptr = root_table_address as *const Table;
            let root_table_ref = unsafe { & *root_table_ptr};
            let root_table_entry = & root_table_ref.content[root_table_index as usize];

            // check if entry is valid or not
            if root_table_entry.check_if_valid() == false { return  Err(errors::TRANS_ERROR_UnallocatedVirtualAddress); }
            else { /* continue */} 

            // get entry under the mid level Page
            let mid_table_ptr = root_table_entry.get_address() as *const Table;
            let mid_table_ref = unsafe { & *mid_table_ptr};
            let mid_table_entry = & mid_table_ref.content[mid_table_index as usize];

            // check if entry is valid or not
            if mid_table_entry.check_if_valid() == false { return  Err(errors::TRANS_ERROR_UnallocatedVirtualAddress); }
            else { /* continue */}

            // get entry under the leaf level Page
            let leaf_table_ptr = mid_table_entry.get_address() as *const Table;
            let leaf_table_ref = unsafe { & *leaf_table_ptr};
            let leaf_table_entry = & leaf_table_ref.content[leaf_table_index as usize];

            // check if entry is valid or not
            if leaf_table_entry.check_if_valid() == false { return  Err(errors::TRANS_ERROR_UnallocatedVirtualAddress); }
            else {  // extract the physical address
                if leaf_table_entry.check_if_branch() == true {    return Err(errors::TRANS_ERROR_InvalidPhysicalAddress);        }
                let physical_page_address = leaf_table_entry.get_address();
                let physical_byte_address = physical_page_address + page_offset;
                return Ok(physical_byte_address);
            }

}

/// This function frees the following pages :   
/// 1. All the physical Pages referenced in the translation tables
/// 2. All the translation tables themselves
pub fn unmap(root_table_address: u64){

    let root_table_ptr = root_table_address as *mut Table;
    let root_table_ref = unsafe { &mut *root_table_ptr};

    // we will loop through the root table entries, one by one. 
    // if a entry points to a mid_level page table... we visit that mid level table
    // once we are in that mid level page, we loop through the entries.
    // if an entry points to a leaf table, we visit that table
    // if a leaf entry points to a physical page, you free that page

    for root_index in (0..512){
        // access the root table entry
        let root_table_entry = &root_table_ref.content[root_index as usize];
        if root_table_entry.check_if_valid() == false { continue; }
        else { // access the mid_level_page
            let mid_table_ptr = root_table_entry.get_address() as *const Table;
            let mid_table_ref = unsafe {& *mid_table_ptr};

            // loop through the mid table entries and when you are done, deallocate the mid Table itself
            for mid_index in (0..512){
                // access each mid entry
                let mid_table_entry = &mid_table_ref.content[mid_index as usize];

                if mid_table_entry.check_if_valid() == false { /* do nothing */}
                else { // access leaf Table Page
                    let leaf_table_ptr = mid_table_entry.get_address() as *const Table;
                    let leaf_table_ref = unsafe { & *leaf_table_ptr};

                    // loop through the entries of the leaf table, and when youre done, deallocate the leaf table itself
                    for leaf_index in (0..512){
                        let leaf_table_entry = &leaf_table_ref.content[leaf_index as usize];
                        if leaf_table_entry.check_if_valid() == false { /* do nothing */}
                        else { // deallocate the physical address being referenced 
                           page_manager::dealloc(leaf_table_entry.get_address() as usize);
                        //    println!(" >>>> I WILL DEALLOCATE : {:016x}", leaf_table_entry.get_address() as usize);
                        }
                    }

                    // deallocate leaf table itself
                    // println!(" >>>> I WILL DEALLOCATE leaf address: {:016x}", leaf_table_ptr as usize);
                    page_manager::dealloc(leaf_table_ptr as usize);
                }
            }

            // deallocate the Mid Table itself
            page_manager::dealloc(mid_table_ptr as usize);
        }
    }

    // deallocate the Root table itself
    page_manager::dealloc(root_table_address as usize);
}

// Function validates a virtual address. It returns true if the address is ...  
// 1. Within a 39 bit range
// 2. Not divisible by 4096
fn validate_virtual_address(address: u64) -> bool{
    // check if address is under the 39-bit threshold
    if address > 2u64.pow(39){ return false; }

    // check if address has 12 trailing zeroes, ie. It is divisible by 4096
    if address % 4096 != 0 { return false; }

    // else return true
    true
}

// checks if a function :   
// 1. is within the 56 bit range
// 2. is divisible by 4096
fn validate_physical_address(address: u64) -> bool{
    // check if address is under the 56-bit threshold
    if address > 2u64.pow(56){ return false; }

    // check if address has 12 trailing zeroes, ie. It is divisible by 4096
    if address % 4096 != 0 { return false; }

    // else return true
    true
}

// makes sure that the access bits are valid
fn validate_access_map(map: u64) -> bool{
    // return true if at least one of the RXW is defined AND all other bits are ZERO
    if (map & 2u64 == 2u64 || map & 4u64 == 4u64 || map & 8u64 == 8u64) & (map & !14u64 == 0)
       { true }
    else { false }
}

/// Shows the virtual-to-physical Table
pub fn show_mappings(root_table_address: u64){

    let root_table_ptr = root_table_address as *mut Table;
    let root_table_ref = unsafe { &mut *root_table_ptr};

    // parts for reconstructiong the virtual address
    let mut virt_address_root : u64;
    let mut virt_address_mid : u64;
    let mut virt_address_leaf : u64;

    for root_index in (0..512){
        virt_address_root = root_index;
        // access the root table entry
        let root_table_entry = &root_table_ref.content[root_index as usize];
        if root_table_entry.check_if_valid() == false { continue; }
        else { // access the mid_level_page
            let mid_table_ptr = root_table_entry.get_address() as *const Table;
            let mid_table_ref = unsafe {& *mid_table_ptr};

            // loop through the mid table entries and when you are done, deallocate the mid Table itself
            for mid_index in (0..512){
                virt_address_mid = mid_index;
                // access each mid entry
                let mid_table_entry = &mid_table_ref.content[mid_index as usize];

                if mid_table_entry.check_if_valid() == false { /* do nothing */}
                else { // access leaf Table Page
                    let leaf_table_ptr = mid_table_entry.get_address() as *const Table;
                    let leaf_table_ref = unsafe { & *leaf_table_ptr};

                    // loop through the entries of the leaf table
                    for leaf_index in (0..512){
                        virt_address_leaf = leaf_index;
                        let leaf_table_entry = &leaf_table_ref.content[leaf_index as usize];
                        if leaf_table_entry.check_if_valid() == false { /* do nothing */}
                        else { // print the physical address being referenced 
                           let combined_virt_address = (virt_address_root << 30) | (virt_address_mid << 21) | (virt_address_leaf << 12);
                           let physical_address = leaf_table_entry.get_address();
                           println!(" \t >>>> {:016x} : {:016x}", combined_virt_address, physical_address);
                        }
                    }

                }
            }
        }
    }
}
