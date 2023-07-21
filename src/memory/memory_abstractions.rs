//! This module does the following  
//! 1. Abstracts the RAM into Pages and Descriptors  
//! 2. Provides functions for maniulating Pages and Descriptors

use volatile_register::RW;

// Abstracting the memory part
// 

// get the heap memory labels from the /asm/memory_export.s assembly file
extern "C"{
    static HEAP_START : usize;
    static HEAP_END : usize;
}

// descriptor possible values
// The last bit shows if a page is taken
// The second_last bit shows if a page is last in the contiguous allocated region
// The third_last bit shows if a page is in the middle of the contiguous allocated region
// The fourth_last bit shows if a page is the first page in the contiguous allocated region

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DescriptorValue{
    Empty = 0b0000_0000,
    FirstAndTaken = 0b0000_1001,
    LastAndTaken = 0b0000_0011,
    MiddleAndTaken = 0b0000_0101,
    FirstAndLast = 0b0000_1011
}

pub struct PageDescriptor{
    value : DescriptorValue
}

impl PageDescriptor {
    // this funtion returns a PageDescriptor that points to an uallocated page
    pub fn new_empty() -> Self{
        PageDescriptor { value: DescriptorValue::Empty }
    }

    // this function creates a PageDescriptor that contains the passed value
    pub fn new(val: DescriptorValue) -> Self{
        PageDescriptor { value: val }
    }

    // get a copy of the descriptor value
    pub fn get_val (&self) -> DescriptorValue{
        self.value
    }

    pub fn set_empty(&mut self){    self.value = DescriptorValue::Empty;    }
    pub fn set_first(&mut self){    self.value = DescriptorValue::FirstAndTaken;    }
    pub fn set_middle(&mut self){    self.value = DescriptorValue::MiddleAndTaken;    }
    pub fn set_last(&mut self){    self.value = DescriptorValue::LastAndTaken;    }
    pub fn set_flast(&mut self){    self.value = DescriptorValue::FirstAndLast;    }
}

// direct map of the page
pub struct PageMMIO{
    content : [RW<u8> ; 4096]
}

impl PageMMIO{
    pub fn clear(&mut self){
        for element in &mut self.content{
            unsafe { element.write(0)}  ;
        }
    }
}


// abstraction over the direct map of the page, it has a mutable reference to all the PageMMIO
// it abstracts 4096 continuous bytes. 
// Its Base address is an address that is divisible by 4096
pub struct Page{
    mmio : &'static mut PageMMIO
}

impl Page{
    // if given an aligned address, this function returns an struct that enables you to manipulate bytes within the page  range
    pub fn new(start_address: usize) -> Page{
        let ptr_to_mmio = start_address as *mut PageMMIO;
        let ref_to_mmio = unsafe {  &mut *(ptr_to_mmio)};
        return Page { mmio: ref_to_mmio };
    }

    // this function zeroes out all the bytes present in the page
    pub fn clear (&mut self) {
        for byte_space in &mut(self.mmio.content){
            unsafe{byte_space.write(0 as u8)};
        } 
    }

}
/// Outlines 
/// 1. the number of descriptors created
/// 2. The number of descriptors
/// 3. The start_address of the Pages. ie ALLOC_START

pub struct SimpleHeapLayout{
    pub num : usize,
    pub alloc_address : usize
}

// Full info about the Heap
// This can be used to monitor the heap OR check for conflicting errors
// Think of it as the a struct containing all the metadata about the Heap
pub struct FullHeapLayout{
    // overall stats of the heap
    heap_start : Option<usize>,
    heap_end : Option<usize>,

    // First segement before the actual pages. This segment is occupied by Descroptors and pad_bytes
    num_of_descriptors : Option<usize>,
    first_descriptor_address : Option<usize>,
    last_descriptor_address : Option<usize>,
    first_padding_address : Option<usize>,
    last_padding_address : Option<usize>,

    // This segemnt deals with the actual pages
    num_of_pages : Option<usize>,
    alloc_start_address : Option<usize>, // address of the first page
    last_page_address : Option<usize>, 
    unused_bytes : Option<usize>, // these are the bytes at the end of the heap that could not aggregate to a full page

    // Real time data. The previous sections are determined during the memory initialization.  
    // From here It is data that changes with respect to allocations and deallocations
    // This data can be used to debug the memory allocatio or fix RAM issues or monitor Heap Usage
    num_of_allocated_pages : Option<usize>, // determined by looking through the descriptors (simple_check)
    num_of_unallocated_pages : Option<usize>, // determined by checkig the descriptors (simple_check)
    num_of_allocations_done : Option<usize>,
    num_of_deallocations_done : Option<usize>, // another way of detrmininf the state of the Heap.
    //  Comparing these differet results can help detect a breach

}




