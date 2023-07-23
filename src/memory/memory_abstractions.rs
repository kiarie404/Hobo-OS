//! This module does the following  
//! 1. Abstracts the RAM into Pages and Descriptors  
//! 2. Provides functions for maniulating Pages and Descriptors

use volatile_register::RW;
use core::fmt::Display;
use core::fmt;

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

// Full info about the Heap
// This can be used to monitor the heap OR check for conflicting errors
// Think of it as the a struct containing all the metadata about the Heap
#[derive(Debug)]
pub struct FullHeapLayout{
    // overall stats of the heap
    pub heap_start : Option<usize>,
    pub heap_end : Option<usize>,
    pub heap_size: Option<usize>,

    // First segement before the actual pages. This segment is occupied by Descroptors and pad_bytes
    pub num_of_descriptors : Option<usize>,
    pub first_descriptor_address : Option<usize>,
    pub last_descriptor_address : Option<usize>,
    pub first_padding_address : Option<usize>,
    pub last_padding_address : Option<usize>,

    // This segemnt deals with the actual pages
    pub num_of_pages : Option<usize>,
    pub alloc_start_address : Option<usize>, // address of the first page
    pub last_page_address : Option<usize>, 
    pub unused_bytes : Option<usize>, // these are the bytes at the end of the heap that could not aggregate to a full page

    // Real time data. The previous sections are determined during the memory initialization.  
    // From here It is data that changes with respect to allocations and deallocations
    // This data can be used to debug the memory allocatio or fix RAM issues or monitor Heap Usage
    pub num_of_allocated_pages : usize, // determined by looking through the descriptors (simple_check)
    pub num_of_unallocated_pages : usize, // determined by checkig the descriptors (simple_check)
    pub num_of_allocations_done : usize,
    pub num_of_deallocations_done : usize, // another way of detrmininf the state of the Heap.
    //  Comparing these differet results can help detect a breach

}

impl FullHeapLayout {
    pub const fn new () -> Self{
        FullHeapLayout { heap_start: None, 
                         heap_end: None, 
                         heap_size: None,
                         num_of_descriptors: None, 
                         first_descriptor_address: None, 
                         last_descriptor_address: None, 
                         first_padding_address: None, 
                         last_padding_address: None, 
                         num_of_pages: None, 
                         alloc_start_address: None, 
                         last_page_address: None, 
                         unused_bytes: None, 
                         num_of_allocated_pages: 0, 
                         num_of_unallocated_pages: 0, 
                         num_of_allocations_done: 0, 
                         num_of_deallocations_done: 0 }
    }


}

impl Display for FullHeapLayout{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         writeln!(f, "\n===========  Heap Layout ========\n");
         writeln!(f, "---------  Overall Heap details --------");
         writeln!(f, "Heap Size : {}", self.heap_size.unwrap());
         writeln!(f, "Heap Start Address : 0x{:x}", self.heap_start.unwrap());
         writeln!(f, "Heap End Address : 0x{:x}", self.heap_end.unwrap());

         writeln!(f, "\n -------- Descriptor Segment ----------\n");
         writeln!(f, "Number of descriptors : {}", self.num_of_descriptors.unwrap());
         writeln!(f, "First Descriptor Address : 0x{:x}", self.first_descriptor_address.unwrap());
         writeln!(f, "Last Descriptor  Address : 0x{:x}", self.last_descriptor_address.unwrap());
         writeln!(f, "First Padding Address : 0x{:x}", self.first_padding_address.unwrap());
         writeln!(f, "Last Padding Address : 0x{:x}", self.last_padding_address.unwrap());

         writeln!(f, "\n -------- Pages Segment ----------\n");
         writeln!(f, "Number of Pages : {}", self.num_of_pages.unwrap());
         writeln!(f, "First Page Address : 0x{:x}", self.alloc_start_address.unwrap());
         writeln!(f, "Last Page Address : 0x{:x}", self.last_page_address.unwrap());
         writeln!(f, "Number of unused Bytes near memory end : {}", self.unused_bytes.unwrap());

         writeln!(f, "\n -------- Allocation Stats Segment ----------\n");
         writeln!(f, "Number of Allocated Pages : {}", self.num_of_allocated_pages);
         writeln!(f, "Number of Un-Allocated Pages : {}", self.num_of_unallocated_pages);
         writeln!(f, "Number of Allocations Done : {}", self.num_of_allocations_done);
         writeln!(f, "Number of Deallocations Done : {}", self.num_of_deallocations_done)
         
    }
}



