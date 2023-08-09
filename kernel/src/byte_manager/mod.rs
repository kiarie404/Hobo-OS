//! This is the byte allocator. It also has the glue code that chooses this byte allocator as the custom allocator that gets to be used by the compiler when dealing with the heap.. 	
//! This MODULE was NOT ORIGINALLY made for this project. It was ported to this project.		  
//! ALL RIGHTS belong to Stephen Marz  


//! -------- metadata --------  
//! Sub-page level: malloc-like allocation system	
//! Stephen Marz  	
//! 7 October 2019	


//! THe byte allocator has been built for the Kernel Heap only.  
//! [undone] : Soon the heap of user programs will have byte allocation too
//! 
//!  
// porting the module....
use crate::page_manager::alloc as zalloc;
use crate::page_manager::align as align_val;
use crate::page_manager::PAGE_SIZE;
use crate::sv39_mmu::Table;
use crate::{print, println};
use core::{mem::size_of, ptr::null_mut};



// what was before has been commented out
// use crate::page::{align_val, zalloc, Table, PAGE_SIZE};
// use core::{mem::size_of, ptr::null_mut};

// 
#[repr(usize)]
enum AllocListFlags {
	Taken = 1 << 63,
}
impl AllocListFlags {
	pub fn val(self) -> usize {
		self as usize
	}
}

struct AllocList {
	pub flags_size: usize,
}
impl AllocList {
	pub fn is_taken(&self) -> bool {
		self.flags_size & AllocListFlags::Taken.val() != 0
	}

	pub fn is_free(&self) -> bool {
		!self.is_taken()
	}

	pub fn set_taken(&mut self) {
		self.flags_size |= AllocListFlags::Taken.val();
	}

	pub fn set_free(&mut self) {
		self.flags_size &= !AllocListFlags::Taken.val();
	}

	pub fn set_size(&mut self, sz: usize) {
		let k = self.is_taken();
		self.flags_size = sz & !AllocListFlags::Taken.val();
		if k {
			self.flags_size |= AllocListFlags::Taken.val();
		}
	}

	pub fn get_size(&self) -> usize {
		self.flags_size & !AllocListFlags::Taken.val()
	}
}

// This is the head of the allocation. We start here when
// we search for a free memory location.
static mut KMEM_HEAD: *mut AllocList = null_mut();
// In the future, we will have on-demand pages
// so, we need to keep track of our memory footprint to
// see if we actually need to allocate more.
static mut KMEM_ALLOC: usize = 0;
static mut KMEM_PAGE_TABLE: *mut Table = null_mut();

// These functions are safe helpers around an unsafe
// operation.
pub fn get_head() -> *mut u8 {
	unsafe { KMEM_HEAD as *mut u8 }
}

pub fn get_page_table() -> *mut Table {
	unsafe { KMEM_PAGE_TABLE as *mut Table }
}

pub fn get_num_allocations() -> usize {
	unsafe { KMEM_ALLOC }
}

/// Initialize kernel's memory 
/// It allocates pages that will be used as kernel Heap and marcates the linked list used by the byte allocator
/// This is not to be used to allocate memory
/// for user processes. If that's the case, use
/// alloc/dealloc from the page crate.
pub fn init_kernel_byte_allocation() {
	unsafe {
		// Allocate kernel pages (KMEM_ALLOC)
		KMEM_ALLOC = 512;  // this is the number of pages that are dedicated to the lernel heap
		let k_alloc_result = zalloc(KMEM_ALLOC);  // actualize those pages and zero them out
		assert!(!k_alloc_result.is_err());                      // make sure the pages were actually allocated
        let first_address:usize = k_alloc_result.unwrap();
		KMEM_HEAD = first_address as *mut AllocList;			// Create the first Alloc List at the very Top
		(*KMEM_HEAD).set_free();						// set the first allocList to be free
		(*KMEM_HEAD).set_size(KMEM_ALLOC * PAGE_SIZE); // make that first allocList declare that the rest of the bytes below it are free ((512 x 4096) - 1)

        // allocate the Page Table that will be used
        let root_table_adress = zalloc(1).expect("unable to allocate space for the kernel root table");
		KMEM_PAGE_TABLE = root_table_adress as *mut Table;  // create The root Page table 
	}
}

/// Allocate sub-page level allocation based on bytes and zero the memory
pub fn kzmalloc(sz: usize) -> *mut u8 {
	// the smallest unit that an be assigned is 8
	// The number of bytes assigned must be a multiple of 8
	let size = align_val(sz, 3);
	let ret = kmalloc(size);

	// if there is free space and kmalloc returns an address
	// loop through the allocated bytes and zero them out
	if !ret.is_null() {
		for i in 0..size {
			unsafe {
				(*ret.add(i)) = 0;
			}
		}
	}

	// return the address of the first allocated byte 
	ret
}

/// Allocate sub-page level allocation based on bytes
pub fn kmalloc(sz: usize) -> *mut u8 {
	unsafe {
		// size of block = (a multiple of 8 no. of blocks) + The ize of the tail block pointer
		let size = align_val(sz, 3) + size_of::<AllocList>(); //

		// we will convert head and tail or any other address into AllocList so that we can use helper functions on them 
		let mut head = KMEM_HEAD;
		// .add() uses pointer arithmetic, so we type-cast into a u8
		// so that we multiply by an absolute size (KMEM_ALLOC *
		// PAGE_SIZE).
		let tail = (KMEM_HEAD as *mut u8).add(KMEM_ALLOC * PAGE_SIZE)
		           as *mut AllocList;

		while head < tail {
			// the initial byte is set to : free and (512 x 4096) . meaning that all kernel pages are free 
			// If the current allocList is above a contiguous set of bytes that are free and are more  or equal to the demanded number of bytes...
			if (*head).is_free() && size <= (*head).get_size() {

				// find out exactly how many bytes are under this AllocList struct
				let chunk_size = (*head).get_size();

				// find out how many free slots will not get occupied 
				let rem = chunk_size - size;

				// mark your territory
				(*head).set_taken();

				// if the remaining space is enough to store an AllocList structure ... 
				if rem > size_of::<AllocList>() {

					// create an Alloc Structure just immediately after the allocated bytes. The number of allocated bytes was "size"
					let next = (head as *mut u8).add(size)
					           as *mut AllocList;
					// There is space remaining here.
					// so update the flag_size information of the new AllocList Structure
					(*next).set_free();
					(*next).set_size(rem);

					// also update the values of the Alloc structure that has had new allocations
					(*head).set_size(size);
				}
				// But if the space is not enough to store an Alloc List Struct
				else {
					// If we get here, take the entire chunk (which includes both free and occupied slots)
					// Those empty slots just go to waste smh
					(*head).set_size(chunk_size);
				}
				// Now we return the address of the first free byte. A byte where we can store data
				// In this case, that byte always comes immediately after the AlloC Struct... hence the .add(1) below
				return head.add(1) as *mut u8;
			}
			// Else if we find that an Alloc list is not free, and the size is less than what we require...
			else {
				// If we get here, what we saw wasn't a free
				// chunk, move on to the next AllocList
				head = (head as *mut u8).add((*head).get_size())
				       as *mut AllocList;
			}
		}
	}
	// If we get here, we didn't find any free chunks--i.e. there isn't
	// enough memory for this. TODO: Add on-demand page allocation.
	null_mut()
}

/// Free a sub-page level allocation
pub fn kfree(ptr: *mut u8) {
	unsafe {
		if !ptr.is_null() {
			let p = (ptr as *mut AllocList).offset(-1);
			if (*p).is_taken() {
				(*p).set_free();
			}
			// After we free, see if we can combine adjacent free
			// spots to see if we can reduce fragmentation.
			coalesce();
		}
	}
}

/// Merge smaller chunks into a bigger chunk
pub fn coalesce() {
	unsafe {
		let mut head = KMEM_HEAD;
		let tail = (KMEM_HEAD as *mut u8).add(KMEM_ALLOC * PAGE_SIZE)
		           as *mut AllocList;

		while head < tail {
			let next = (head as *mut u8).add((*head).get_size())
			           as *mut AllocList;
			if (*head).get_size() == 0 {
				// If this happens, then we have a bad heap
				// (double free or something). However, that
				// will cause an infinite loop since the next
				// pointer will never move beyond the current
				// location.
				break;
			}
			else if next >= tail {
				// We calculated the next by using the size
				// given as get_size(), however this could push
				// us past the tail. In that case, the size is
				// wrong, hence we break and stop doing what we
				// need to do.
				break;
			}
			else if (*head).is_free() && (*next).is_free() {
				// This means we have adjacent blocks needing to
				// be freed. So, we combine them into one
				// allocation.
				(*head).set_size(
				                 (*head).get_size()
				                 + (*next).get_size(),
				);
			}
			// If we get here, we might've moved. Recalculate new
			// head.
			head = (head as *mut u8).add((*head).get_size())
			       as *mut AllocList;
		}
	}
}

/// For debugging purposes, print the kmem table
pub fn print_table() {
	unsafe {
		let mut head = KMEM_HEAD;
		let tail = (KMEM_HEAD as *mut u8).add(KMEM_ALLOC * PAGE_SIZE)
		           as *mut AllocList;
		while head < tail {
			println!(
			         "{:p}: Length = {:<10} Taken = {}",
			         head,
			         (*head).get_size(),
			         (*head).is_taken()
			);
			head = (head as *mut u8).add((*head).get_size())
			       as *mut AllocList;
		}
	}
}

// ///////////////////////////////////
// / GLOBAL ALLOCATOR
// ///////////////////////////////////

// The global allocator allows us to use the data structures
// in the core library, such as a linked list or B-tree.
// We want to use these sparingly since we have a coarse-grained
// allocator.
use core::alloc::{GlobalAlloc, Layout};

// The global allocator is a static constant to a global allocator
// structure. We don't need any members because we're using this
// structure just to implement alloc and dealloc.
struct OsGlobalAlloc;

unsafe impl GlobalAlloc for OsGlobalAlloc {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		// We align to the next page size so that when
		// we divide by PAGE_SIZE, we get exactly the number
		// of pages necessary.
		kzmalloc(layout.size())
	}

	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
		// We ignore layout since our allocator uses ptr_start -> last
		// to determine the span of an allocation.
		kfree(ptr);
	}
}

#[global_allocator]
/// Technically, we don't need the {} at the end, but it
/// reveals that we're creating a new structure and not just
/// copying a value.
static GA: OsGlobalAlloc = OsGlobalAlloc {};

#[alloc_error_handler]
/// If for some reason alloc() in the global allocator gets null_mut(),
/// then we come here. This is a divergent function, so we call panic to
/// let the tester know what's going on.
pub fn alloc_error(l: Layout) -> ! {
	panic!(
	       "Allocator failed to allocate {} bytes with {}-byte alignment.",
	       l.size(),
	       l.align()
	);
}
