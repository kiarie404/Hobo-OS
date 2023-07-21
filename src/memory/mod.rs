
//! This module declares funtions that manipulate the memory abstractions

mod memory_abstractions;
mod memory_errors;
use memory_abstractions::{SimpleHeapLayout, DescriptorValue, PageDescriptor, Page, PageMMIO};
use memory_errors::{MemoryAllocatioErrors, MemoryDeallocatioError};
use core::mem::size_of;

// get the heap memory labels from the /asm/memory_export.s assembly file
extern "C"{
    static HEAP_START : usize;
    static HEAP_END : usize;
}
// Borrow the imported Variables to reduce the number of unsafe blocks 
static START : &usize = unsafe{&HEAP_START};
static END   : &usize = unsafe{&HEAP_END};
static mut ALLOC_START : usize = 0;
static mut NUM_DP : usize = 0; // the number of pages created. THe number of pages also equals the number of descriptors

// calculates total Heap size in bytes
fn get_heap_size() -> usize{
    let heap_memory_size = ((*END + 1) - *START) * size_of::<u8>(); // size in bytes
    return heap_memory_size;
}

/// This function uses the Page and Descriptor structs to define the actual Heap layout
fn init_memory_abtraction(){
    // clear memory, make every byte in the heap a zero
    clear_entire_heap(); 
    // Determine Heap layout, and update the global variables {ALLOC_START, NUM_DP}.
    determine_heap_layout();

}

// This function traverses the entire heap and makes sure each value under each address is zero
fn clear_entire_heap(){
    let mut index = *START;
    while index <= *END {
        let ptr_to_byte = index as *mut u8;
        unsafe { ptr_to_byte.write_volatile(0)};
        index = index + size_of::<u8>();
    }
}

// aligns memory to the specified order
fn align (val: usize, order: usize) -> usize{
    let addition_mask : usize = 1 << order;    // eg if we need to find things in order of 2 ie 2^2, the number will always have 2 zeroes at the LSB
    let over_board : usize = val + addition_mask; // so when we get a non_mutiple number, we make sure it passes the next multiple.  
    let cut_mask : usize = !0 << order; // a mask that will be used to make the last 2 bits become zeroes
    let result: usize = over_board & cut_mask; // replace the last 2 bits with zeroes
    return result;
}

// This function returns the SimpleHeapLayout, outliing the numer of Pages to be created, Descriptors and the actial ALLOC address
// It also modifies the static ALLOC_START address
fn determine_heap_layout() -> SimpleHeapLayout{
    // calculate the MAXIMUM number of dexcriptors and Pages that can be made in the heap space
    // MAX number = (heap_memory_size) / (sizeof_page_and_descriptor)
    let heap_memory_size = get_heap_size(); // size in bytes
    let size_of_page_and_descriptor: usize = 4096 + 1; // size in bytes ie. (Page size + Descriptor size)
    let max_num_dp = heap_memory_size / size_of_page_and_descriptor;
    let unused_bytes = heap_memory_size % size_of_page_and_descriptor;

    // The ALLOC_START is aligned to 4096
    // This is the place where the pages to be allocated start
    // This position comes after the decscriptors
    let address_afer_last_descriptor : usize = (*START + max_num_dp);
    let alloc_start : usize = align(address_afer_last_descriptor, 12) ;

    // Now with the Alloc_start position known, we can get the actual number of pages and descriptors
    let actual_num_pages : usize = ((*END + 1) - alloc_start) * size_of::<u8>();

    // after determining the Heap Layout, update the Global Variable {ALLOC_START, NUM_DP}
    unsafe {
        ALLOC_START = alloc_start.clone();
        NUM_DP = actual_num_pages.clone();
    }

    return SimpleHeapLayout{  num: actual_num_pages, alloc_address: alloc_start };
}

// takes in the number of requested pages and returns the address of the first memory byte of the contiguous allocation
// returns an error if requested zero pages
// returns an error if No contiguous free space is found
fn alloc(req_pages: usize) -> Result<usize, MemoryAllocatioErrors>{
    // check if required pages is zero. If its zero, throw an error...
    if req_pages == 0 { return Err(MemoryAllocatioErrors::ZeroPagesRequested("Zero pages were requested from the allocator"));}
    else { // traverse the array of descriptors, lookng for a contiguos free space
        let search_result = find_first_contiguous(req_pages);
        match search_result {
           Some(descriptor_index) => return Ok(descriptor_index),
           None => return Err(MemoryAllocatioErrors::NoFreeContiguousSpace("No Free contiguous pages were found")) 
        }
    }
}

/// This function ...   
/// 1. takes the Address of the first Page of a contiguous page allocation
/// and frees all of the associated contiguous pages. Freeing here means ZEROING all bytes of the page.   
/// 2. It updates the corresponding descriptors associated with the freed Pages  
/// 
/// Errors thrown include:  
/// 1. Address passed to the function is not the first in its associated contiguous allocation
/// 2. Address passed is not a valid address. because it is not found within the Heap Page section, or it is not a Page's first byte. 
fn dealloc(page_addr: usize) -> Result<(), MemoryDeallocatioError>{
    // validate page_addr... and move on to deallocation
    if check_if_page_within_heap(page_addr) == false { 
        return Err(MemoryDeallocatioError::NonHeapAddressFound("Page address is not within the Heap")); }
    else if check_if_page_top_addr(page_addr) == false {
        return Err(MemoryDeallocatioError::PageAddressIsMiddlePage("The address is not divisible by 4096"));
    }
    else if check_if_page_is_first(page_addr) == false {
        return Err(MemoryDeallocatioError::PageNotLeading("The Page address references to a Page that is not the leading page in the contiguous group of pages"));
    }
    else {

    }

unimplemented!()
}

// Finds page index ; its location in the array of pages.  
// It assumes that the First page is at ALLOC start and has the index 0 
fn get_page_index(page_addr: usize) -> usize{
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
    let page_index = get_page_index(page_addr);
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
fn check_if_page_top_addr(page_addr: usize) -> bool{
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
fn empty_group_of_descriptors(index: usize) -> Result<usize, MemoryDeallocatioError>{
    unsafe{
     // get variables ready
     let base_ptr = HEAP_START as *mut PageDescriptor;
     let subject_ptr = base_ptr.add(index);
     let subject_ref = &mut *subject_ptr;
     let subject_val = subject_ref.get_val();

     match subject_val {
         DescriptorValue::Empty => return Err(MemoryDeallocatioError::Other("Tried to free an empty Descriptor")),
         DescriptorValue::LastAndTaken => return Err(MemoryDeallocatioError::Other("The Passed index was not a leading descriptor")),
         DescriptorValue::MiddleAndTaken => return Err(MemoryDeallocatioError::Other("The Passed index was not a leading descriptor")),
         DescriptorValue::FirstAndLast => {
            subject_ref.set_empty(); // free that descriptor
            return Ok(1);
         },
         DescriptorValue::FirstAndTaken => {
            // loop until you find the last Descriptor for that allocation. 
            // make sure the descriptors are in order
            let mut count: usize = 1;
            loop {
                let next_desc_ptr = subject_ptr.add(count);
                let next_desc_ref = &mut *next_desc_ptr;
                let next_desc_val = next_desc_ref.get_val();
                match next_desc_val {
                    DescriptorValue::Empty => return Err(MemoryDeallocatioError::Other("Misplaced Empty Descriptor")),
                    DescriptorValue::LastAndTaken => { break; }, // break out, the order of descriptors is fine
                    DescriptorValue::FirstAndLast => return Err(MemoryDeallocatioError::Other("Misplaced FirstAndLast Descriptor")),
                    DescriptorValue::FirstAndTaken => return Err(MemoryDeallocatioError::Other("Misplaced FirstAndTaken Descriptor")),
                    DescriptorValue::MiddleAndTaken => {
                        count = count + 1; // we only get out of this loop if we get an error or if we reach the last descriptor
                    },
                }
            }

            // If the order is fine, then clear the contiguous descriptors
            count = 0;
            loop {
                let next_desc_ptr = subject_ptr.add(count);
                let next_desc_ref = &mut *next_desc_ptr; 
                if next_desc_ref.get_val() == DescriptorValue::LastAndTaken {
                    next_desc_ref.set_empty();
                    return Ok(count+1);
                }  
                else {  next_desc_ref.set_empty(); }         
            }
         }// end of looping.
     }
     

    }
}

// this function receives a lead page address and the number of contiguous pages that need to be freed.  
// It then Zeroes all the pages involved
fn empty_group_of_pages(page_addr: usize, num_pages: usize){
    unsafe {
        let base_page_ptr = page_addr as *mut PageMMIO;
        let base_page_ref = &mut *base_page_ptr;

        for index in 0..num_pages{
            let subject_ptr = base_page_ptr.add(index);
            let subject_ref = &mut *subject_ptr;
            subject_ref.clear();
        }
    }
}



// this function takes the pointer_address to the first element of an array of u8s AND the number of bytes required contiguously
// and returns the first array index of the first sufficient contiguous space
// or it returns a None
fn find_first_contiguous( req_val: usize) -> Option<usize>{
    let base_ptr = unsafe{HEAP_START as *const PageDescriptor}; // pointer to the start of the Array of PageDescriptors
    let mut count : usize = 0; // count of a continuous empty streak found
    let num_of_descriptors = unsafe{   NUM_DP.clone()}; // number of descriptors

    for index in (0..num_of_descriptors){ // parse all bytes while setting and resetting the count
        let current_ptr = unsafe {base_ptr.add(index)};
        let page_descriptor = unsafe{& *current_ptr};
        let descriptor_val = page_descriptor.get_val();
        if descriptor_val == DescriptorValue::Empty { // if descriptor is free ...
            count = count + 1;
            if count == req_val {   return Some(index-(req_val - 1)); }
        } 
        else {  count = 0; } // reset count  
    }

    return None;   
}

count