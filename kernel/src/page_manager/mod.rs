
//! This module declares funtions that manipulate the memory abstractions
//! More Specifically, It provides :
//! 1. Memory initialization function
//! 2. Page Allocation
//! 3. Page Deallocation
//! 4. Heap Monitoring  

mod memory_abstractions;
mod memory_errors;
mod tests; // tests that test the functions defined in this module


use memory_abstractions::{FullHeapLayout, DescriptorValue, PageDescriptor, Page, PageMMIO};
use memory_errors::{MemoryDeallocationError, MemoryAllocationError};
use core::mem::size_of;
use crate::{print, println};


// get the heap memory labels from the /asm/memory_export.s assembly file
extern "C"{
    static HEAP_START : usize;
    static HEAP_END : usize;
}
// Borrow the imported Variables to reduce the number of unsafe blocks 
static START : &usize = unsafe{&HEAP_START};
static END   : &usize = unsafe{&HEAP_END};
static mut ALLOC_START : usize = 0;
static mut NUM_DP : usize = 0; // the number of pages in the heap. THe number of pages also equals the number of descriptors
pub const PAGE_SIZE: usize = 4096;

// The FullHeapLayout contains all metadata about the Heap stats, The allocations and deallocations
// The FullHeapLayout contents get updated by the following functions :
// 1. The init_memory_abtraction
// 2. The alloc function
// 3. The Dealloc function
static mut HEAP_LAYOUT : FullHeapLayout = FullHeapLayout::new();

// calculates total Heap size in bytes
fn get_heap_size() -> usize{
    let heap_memory_size = ((*END + 1) - *START) * size_of::<u8>(); // size in bytes
    return heap_memory_size;
}

/// This function does the following:   
/// 1. Clears the entire heap
/// 2. Divides the heap into Segments : Descriptors and Pages   
/// You can view the layout of the heap using the "fn show_layout()" function
pub fn init_memory(){
    println!(">>>> Initializing memory"); // [remove]
    // clear memory, make every byte in the heap a zero
    clear_entire_heap(); // [undone] : this operation is expensive, find a way to fix it

    // Determine Heap layout, and update the global variables {ALLOC_START, NUM_DP}.
    determine_heap_layout();

}

// This function traverses the entire heap and makes sure each value under each address is zero
// [undone] : this operation is very very expensive... is it even necerrary?
fn clear_entire_heap(){
    let mut index = unsafe{HEAP_START.clone()};
    while index <= unsafe{HEAP_END} {
        let ptr_to_byte = index as *mut u8;
        unsafe { ptr_to_byte.write_volatile(0)};
        index = index + size_of::<u8>();
    }
}

/// aligns memory to the specified order
pub fn align (val: usize, order: usize) -> usize{
    let addition_mask : usize = 1 << order;    // eg if we need to find things in order of 2 ie 2^2, the number will always have 2 zeroes at the LSB
    let over_board : usize = val + addition_mask; // so when we get a non_mutiple number, we make sure it passes the next multiple.  
    let cut_mask : usize = !0 << order; // a mask that will be used to make the last 2 bits become zeroes
    let result: usize = over_board & cut_mask; // replace the last 2 bits with zeroes
    return result;
}

// This function updates the HEAPLAYOUT static struct
// It can only be called after the memory has been initialized, otherwise, if called before that, it will produce crappy data
fn determine_heap_layout(){
    // calculate the MAXIMUM number of dexcriptors and Pages that can be made in the heap space
    // MAX number = (heap_memory_size) / (sizeof_page_and_descriptor)
    let heap_memory_size = get_heap_size(); // size in bytes
    let size_of_page_and_descriptor: usize = 4096 + 1; // size in bytes ie. (Page size + Descriptor size)
    let max_num_dp = heap_memory_size / size_of_page_and_descriptor;
    let unused_bytes = heap_memory_size % size_of_page_and_descriptor;

    // The ALLOC_START is aligned to 4096
    // This is the place where the pages to be allocated start
    // This position comes after the decscriptors
    let address_after_last_descriptor : usize = (*START + max_num_dp);
    let alloc_start : usize = align(address_after_last_descriptor, 12) ;

    // Now with the Alloc_start position known, we can get the actual number of pages and descriptors
    let actual_num_pages : usize = ((*END + 1) - alloc_start) * size_of::<u8>();
    let last_page_address = alloc_start + (actual_num_pages - 1);

    // after determining the Heap Layout, update the HeapLayout structure
    unsafe {
        HEAP_LAYOUT.heap_start = Some(HEAP_START);
        HEAP_LAYOUT.heap_end   = Some(HEAP_END);
        HEAP_LAYOUT.heap_size = Some(heap_memory_size);
        HEAP_LAYOUT.num_of_descriptors = Some(actual_num_pages);
        HEAP_LAYOUT.first_descriptor_address = Some(HEAP_START);
        HEAP_LAYOUT.last_descriptor_address = Some(address_after_last_descriptor - 1);
        HEAP_LAYOUT.first_padding_address = Some(address_after_last_descriptor);
        HEAP_LAYOUT.last_padding_address = Some(alloc_start - 1);
        HEAP_LAYOUT.alloc_start_address = Some(alloc_start);
        HEAP_LAYOUT.num_of_pages = Some(actual_num_pages);
        HEAP_LAYOUT.last_page_address = Some(last_page_address);
        HEAP_LAYOUT.unused_bytes = Some(HEAP_END - last_page_address);


        ALLOC_START = alloc_start.clone();
        NUM_DP = actual_num_pages.clone();

        // show_layout();
    }

    
}

/// This function takes in the number of requested pages and returns the memory address of the first page of the contiguous page allocation     
/// alloc() returns an error if requested zero pages    
/// it also returns an error if No sufficient contiguous free space is found. At that point, you may need to fragment things
pub fn alloc(req_pages: usize) -> Result<usize, MemoryAllocationError>{
    // println!(">>>> Allocating {} Pages....", req_pages);  // [test] Add this line when running integration tests 9 and below
    // check if required pages is zero. If its zero, throw an error...
    if req_pages == 0 { return Err(MemoryAllocationError::ZeroPagesRequested("Zero pages were requested from the allocator"));}
    else { // traverse the array of descriptors, lookng for a contiguous free space
        let search_result = find_first_contiguous(req_pages); // return address of first contiguous space
        let mut first_descriptor_index : usize;

        match search_result {
           Some(descriptor_index) => first_descriptor_index = descriptor_index,
           None => return Err(MemoryAllocationError::NoFreeContiguousSpace("No Free contiguous pages were found")) 
        }

        // Now that we have the index of the first Descriptors of the contiguous space....
        fill_descriptors(first_descriptor_index, req_pages); // fill the decriptors with appropriate values

        // update the HeapLayout
        // THe Allocator DOES NOT directly update HEAP_LAYOUT.num_of_allocated_pages, it calls the update_heap_page_states function
        // It only updates HEAP_LAYOUT.num_of_allocations_done
        // This is for security reasons
        unsafe{HEAP_LAYOUT.num_of_allocations_done = (HEAP_LAYOUT.num_of_allocations_done ) + req_pages;}
        

        // Finally return the address of the Page that directly corresponds with the First Descriptor
        let page_address = get_page_addr_from_page_index(first_descriptor_index);
        return  Ok(page_address);
    }
}

// This function fills a contiguous group of descriptors with values. It makes sure the order of values in not contradictory:
// Eg : [First, First ] would never occur
fn fill_descriptors(first_descriptor_index: usize, req_pages: usize){
    unsafe {
        let base_descriptor_ptr = HEAP_START as *mut PageDescriptor; // access the array of descriptors
        let first_descriptor_ptr = base_descriptor_ptr.add(first_descriptor_index); // access the first descriptor to be allocated 
        let first_descriptor_ref = &mut *first_descriptor_ptr; 

        // if the requeired pages were a MAX of 1, just give the descriptor the value : FirstAndLast
        if req_pages == 1 { first_descriptor_ref.set_flast();   }
        else {
            let last_index = req_pages - 1;
            for index in (0..req_pages){
               let desc_ptr = first_descriptor_ptr.add(index);
               let desc_ref = &mut *desc_ptr;

               if (index == 0){
                    desc_ref.set_first();
                    // println!("   >>>> Ox{:x} : {:?}",desc_ptr as usize, desc_ref.get_val() ); [remove]
               }

               else if (index < last_index){
                    desc_ref.set_middle();
                    // println!("   >>>> Ox{:x} : {:?}",desc_ptr as usize, desc_ref.get_val() );  [remove]
               }

               else {
                    desc_ref.set_last();
                    // println!("   >>>> Ox{:x} : {:?}",desc_ptr as usize, desc_ref.get_val() );  [remove]
               }

            }
        }
    

    }
}

// THis function updates the numbbers of the alloc/dealloc pages in the HEAPLAYOUT
fn update_heap_page_states(){
    let mut num_of_allocated_pages : usize;
    let mut num_of_unallocated_pages: usize;
    
    (num_of_allocated_pages, num_of_unallocated_pages) = get_page_counts();
    unsafe{
        HEAP_LAYOUT.num_of_allocated_pages = num_of_allocated_pages;
        HEAP_LAYOUT.num_of_unallocated_pages = num_of_unallocated_pages;
    }
}

// counts the number of allocated and unallocated descriptors 
// It returns (allocated, unallocated)
fn get_page_counts() -> (usize, usize){
    println!(">>>> Parsing the heap and determining the number of both allocated and unallocated pages...this will take some time..."); // [remove]
    println!("\t >>>> To reduce the time, how about reducing the number of descriptors to be passed... for test purposes only.");
    unsafe{
        let base_desc_ptr = HEAP_START as *const PageDescriptor;
        let mut count_of_allocated: usize = 0;
        let mut count_of_unallocated : usize = 0;
        for index in 0..10000{   // [test] : You can reduce the number of descriptors to be parsed. efault Value NUM_DP
            let current_desc_ptr = base_desc_ptr.add(index);
            let current_desc_ref = & *current_desc_ptr;
            match current_desc_ref.get_val() {
                DescriptorValue::Empty => count_of_unallocated = count_of_unallocated + 1,
                DescriptorValue::FirstAndLast => count_of_allocated = count_of_allocated + 1,
                DescriptorValue::FirstAndTaken => count_of_allocated = count_of_allocated + 1,
                DescriptorValue::MiddleAndTaken => count_of_allocated = count_of_allocated + 1,
                DescriptorValue::LastAndTaken => count_of_allocated = count_of_allocated + 1,
            }   
        }
        
        println!("\t >>>> Parsing complete");
        return (count_of_allocated,count_of_unallocated);
    }
    
}

/// This function checks if the Descriptors are arranged well, that their ordr is not messed up :   
/// It makes sure that :  
    /// All FirstTaken  Descriptorsare followed by  Last, Middle,   BUT NOT Empty,  FirstTaken FirstLast
    /// All Middle Descriptors are followed by LastTaken or Middle  BUT NOT empty, FirstLast, FirstTaken 
    /// All FirstLast Descriptors are followed by Empty, Flast, First BUT NOT Last, Middle, 
    /// All LastTaken Descriptors are followed by Empty, First, Flast BUT NOT Last, Middle, 
    /// All Empty Descriptors are followed by Empty, FirstTaken, Flast  BUT NOT Last, Middle
pub fn check_descriptor_ordering() -> bool{
    unsafe{
        println!(">>>> Checking order of descriptors...");
        let base_desc_ptr = HEAP_START as *const PageDescriptor;
        let base_desc_ref = & *base_desc_ptr;

        for index in 0..NUM_DP{
            let current_desc_ptr = base_desc_ptr.add(index);
            let current_desc_ref = & *current_desc_ptr;
            let current_desc_val = current_desc_ref.get_val();
            let next_desc_ptr = current_desc_ptr.add(1);
            let next_desc_ref = & *next_desc_ptr;
            let next_desc_val = next_desc_ref.get_val();

            let p_address = get_page_addr_from_page_index(index);
            // println!(">>>> Checking address: {:016x} : {} <> referencing address : {:016x}",current_desc_ptr as usize, current_desc_val, p_address);
            // deal with FirstTakens
            match current_desc_val {
                DescriptorValue::FirstAndTaken => {
                    match next_desc_val {
                        DescriptorValue::FirstAndTaken => return false,
                        DescriptorValue::FirstAndLast => return false,
                        DescriptorValue::MiddleAndTaken => {/* do nothing */},
                        DescriptorValue::LastAndTaken => {/* do nothing */},
                        DescriptorValue::Empty => return false,
                    }
                },

                DescriptorValue::FirstAndLast => {
                    match next_desc_val {
                        DescriptorValue::FirstAndTaken => {/* do nothing */},
                        DescriptorValue::FirstAndLast => {/* do nothing */},
                        DescriptorValue::MiddleAndTaken => return false,
                        DescriptorValue::LastAndTaken => return false,
                        DescriptorValue::Empty => {/* do nothing */},
                    }
                },

                DescriptorValue::MiddleAndTaken => {
                    match next_desc_val {
                        DescriptorValue::FirstAndTaken => return false,
                        DescriptorValue::FirstAndLast => return false,
                        DescriptorValue::MiddleAndTaken => {/* do nothing */},
                        DescriptorValue::LastAndTaken => {/* do nothing */},
                        DescriptorValue::Empty => return false,
                    }
                },

                DescriptorValue::LastAndTaken => {
                    match next_desc_val {
                        DescriptorValue::FirstAndTaken => {/* do nothing */},
                        DescriptorValue::FirstAndLast => {/* do nothing */},
                        DescriptorValue::MiddleAndTaken => return false,
                        DescriptorValue::LastAndTaken => return false,
                        DescriptorValue::Empty => {/* do nothing */},
                    }
                },

                DescriptorValue::Empty => {
                    match next_desc_val {
                        DescriptorValue::FirstAndTaken => {/* do nothing */},
                        DescriptorValue::FirstAndLast => {/* do nothing */},
                        DescriptorValue::MiddleAndTaken => return false,
                        DescriptorValue::LastAndTaken => return false,
                        DescriptorValue::Empty => {/* do nothing */},
                    }
                },
            }


        }

        // if no false return happened
        println!("\t >>>> Order of descriptors is fine");
        return true;
    }

    // unimplemented!()
}


/// This function ...   
/// 1. takes the Address of the first Page of a contiguous page allocation
/// and frees all of the associated contiguous pages. Freeing here means ZEROING all bytes of the page.   
/// 2. It updates the corresponding descriptors associated with the freed Pages  
/// 
/// Errors thrown include:  
/// 1. Address passed to the function is not the first in its associated contiguous allocation
/// 2. Address passed is not a valid address. because it is not found within the Heap Page section, or it is not a Page's first byte. 
pub fn dealloc(page_addr: usize) -> Result<(), MemoryDeallocationError>{
    println!(">>>> Deallocating contiguous memory at address : 0x{:x}...", page_addr);
    // validate page_addr... and move on to deallocation
    if check_if_page_within_heap(page_addr) == false { 
        return Err(memory_errors::NON_HEAP_ADDRESS); }
    else if check_if_page_addr(page_addr) == false {
        return Err(memory_errors::NON_PAGE_ADDRESS);
    }
    else if check_if_page_is_first(page_addr) == false {
        return Err(memory_errors::PAGE_NOT_LEADING);
    }
    else {
        let page_index = get_page_index_from_addr(page_addr);
        let deallocate_desc_results = empty_group_of_descriptors(page_index);
        let mut emptied_descriptors: usize;
        match deallocate_desc_results {
            Ok(num) => emptied_descriptors = num,
            Err(error) => return Err(error)
        }

        empty_group_of_pages(page_addr, emptied_descriptors);

        // UPDATE HEAPLAYOUT
        // THe DeAllocator DOES NOT directly update HEAP_LAYOUT.num_of_unallocated_pages,
        // It only updates HEAP_LAYOUT.num_of_deallocations_done
        // This is for security reasons
        unsafe{HEAP_LAYOUT.num_of_deallocations_done = HEAP_LAYOUT.num_of_deallocations_done + emptied_descriptors;}
        return Ok(());
        
    }

// unimplemented!()
}

// Finds page index ; its location in the array of pages.  
// It assumes that the First page is at ALLOC start and has the index 0 
fn get_page_index_from_addr(page_addr: usize) -> usize{
    // assuming the first page is index 0... the page index of passed page is...
    // index 0 is at page_address : ALLOC_addr + (4096 * 0)
    // index 1 is at page_address : ALLOC_addr + (4096 * 1)
    // index 2 is at page_address : ALLOC_addr + (4096 * 2) ... 
    // If we apply some algebra ....
    // It means that page_address = ALLOC_addr + (4096 * page_index)
    let page_index = unsafe {(page_addr - ALLOC_START) / 4096};
    return page_index;
}

// function checks if the address is the first in the allocation    
// This function assumes that the passed address is a valid page address    
fn check_if_page_is_first(page_addr: usize) -> bool{
    let page_index = get_page_index_from_addr(page_addr);
    // it is assumed that the page index is equal to the descriptor index, 
    let desc_index = page_index;
    let subject_descriptor_ptr = (START + desc_index) as *const PageDescriptor;
    let subject_decriptor_ref = unsafe{& *subject_descriptor_ptr};
    // check the value of the descriptor
    if (subject_decriptor_ref.get_val() == DescriptorValue::FirstAndTaken) || 
       (subject_decriptor_ref.get_val() == DescriptorValue::FirstAndLast){
            return true;
       }
    else {  return false;   }
}

// check if  Page address is within the memory space reserved for Pages
fn check_if_page_within_heap(page_addr: usize) -> bool{
    if page_addr <= *END && page_addr >= unsafe{ ALLOC_START }{
        return true;
    }
    else {  return false;   }
}

// checks if the page address references the start of a page and not within it
// An address is at the start if it is a multiple of 4096
fn check_if_page_addr(page_addr: usize) -> bool{
    let remainder = page_addr % 4096;
    if remainder > 0 {  return false; }
    else { return true; }
}

// THis function receives the index of a descriptor. The index is supposed to be the of a desriptor That is a First.    
// It empties all the associated descriptors and returns number of descriptors freed 
// You can then pass this number to the page_freer
// Errors are thrown when :  
// 1. The Index passed is not pointing to a first page
// 2. THe contiguous descriptors are not in order : eg fisrt-middle-NO_LAST  OR first-NO_LAST 
fn empty_group_of_descriptors(index: usize) -> Result<usize, MemoryDeallocationError>{
    println!(">>>> Deallocating descriptors....");
    unsafe{
     // get variables ready
     let base_ptr = HEAP_START as *mut PageDescriptor;
     let subject_ptr = base_ptr.add(index);
     let subject_ref = &mut *subject_ptr;
     let subject_val = subject_ref.get_val();

     match subject_val {
         DescriptorValue::Empty => return Err(MemoryDeallocationError::Other("Tried to free an empty Descriptor")),
         DescriptorValue::LastAndTaken => return Err(MemoryDeallocationError::Other("The Passed index was not a leading descriptor")),
         DescriptorValue::MiddleAndTaken => return Err(MemoryDeallocationError::Other("The Passed index was not a leading descriptor")),
         DescriptorValue::FirstAndLast => {
            subject_ref.set_empty(); // free that descriptor
            return Ok(1);
         },
         DescriptorValue::FirstAndTaken => {
            // loop until you find the last Descriptor for that allocation. 
            // make sure the descriptors are in order
            let mut count: usize = 1;
            println!("\t >>> Doing the order check...");
            loop {
                let next_desc_ptr = subject_ptr.add(count);
                let next_desc_ref = &mut *next_desc_ptr;
                let next_desc_val = next_desc_ref.get_val();
                match next_desc_val {
                    DescriptorValue::Empty => return Err(MemoryDeallocationError::Other("Misplaced Empty Descriptor")),
                    DescriptorValue::LastAndTaken => { break; }, // break out, the order of descriptors is fine
                    DescriptorValue::FirstAndLast => return Err(MemoryDeallocationError::Other("Misplaced FirstAndLast Descriptor")),
                    DescriptorValue::FirstAndTaken => return Err(MemoryDeallocationError::Other("Misplaced FirstAndTaken Descriptor")),
                    DescriptorValue::MiddleAndTaken => {
                        count = count + 1; // we only get out of this loop if we get an error or if we reach the last descriptor
                    },
                }
            }

            // If the order is fine, then clear the contiguous descriptors
            count = 0;
            println!("\t >>> Doing the emptying loop...");
            loop {
                let current_desc_ptr = subject_ptr.add(count);
                let current_desc_ref = &mut *current_desc_ptr; 
                if current_desc_ref.get_val() == DescriptorValue::LastAndTaken {
                    current_desc_ref.set_empty();
                    println!("\t >>>> Finished Emptying Descriptors....");
                    return Ok(count+1); // returns the number of descriptors deallocated
                }  
                else {  current_desc_ref.set_empty();
                        count = count + 1;  }         
            }
         }// end of looping.
     }
     

    }
}

// this function receives a lead page address and the number of contiguous pages that need to be freed.  
// It then Zeroes all the pages involved
fn empty_group_of_pages(page_addr: usize, req_pages: usize){
    unsafe {
        let base_page_ptr = page_addr as *mut PageMMIO;
        let base_page_ref = &mut *base_page_ptr;
        let subject_index = get_page_index_from_addr(page_addr);
        let subject_base_ptr = base_page_ptr.add(subject_index);
        let subject_base_ref = &mut *subject_base_ptr;

        for index in 0..req_pages{
            let subject_ptr = subject_base_ptr.add(index);
            let subject_ref = &mut *subject_base_ptr;
            subject_ref.clear();
        }
    }
}

fn get_page_addr_from_page_index(index: usize) -> usize { // returns page address associated with the descriptor index
    let page_address = unsafe{(ALLOC_START as *const PageMMIO).add(index)}; 
    return page_address as usize;
}



// This function takes in the number of requested pages
// It parses the descriptors, looking for a sufficient contiguous spae
// If it finds space, it returns the Descriptor index of the leading descriptor of the contiguous space
// If it doesn't find Space, It returns NONE
fn find_first_contiguous( req_pages: usize) -> Option<usize>{
    let base_ptr = unsafe{HEAP_START as *const PageDescriptor}; // pointer to the start of the Array of PageDescriptors
    let mut count : usize = 0; // count of a continuous empty streak found
    let num_of_descriptors = unsafe{  NUM_DP }; // number of descriptors

    for index in (0..num_of_descriptors){ // parse all Descriptors while setting and resetting the count
        let current_ptr = unsafe {base_ptr.add(index)}; // get pointer of current descriptor
        let current_descriptor = unsafe{& *current_ptr}; // get reference of current descriptor
        let descriptor_val = current_descriptor.get_val();
        if descriptor_val == DescriptorValue::Empty { // if descriptor is free ...
            count = count + 1;
            if count == req_pages {   
                // println!(">>>> First empty descriptor was at index {}",index-(req_pages - 1) );
                return Some(index-(req_pages - 1)); }
        } 
        else {  count = 0; } // reset count  
    }

    return None;   
}

/// This fuction displays the current state of the Heap.  
/// You can use this information to debug the Heap iitialization, allocation and deallocatio
pub fn show_layout(){
    update_heap_page_states(); 
    println!(">>>> Getting information about the Heap Layout.... ");
    println!("{}", unsafe{&HEAP_LAYOUT});
}
