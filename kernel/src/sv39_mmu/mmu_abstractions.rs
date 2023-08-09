//! Here are the abstractions of the MMU related hardware   
//! More specifically, the SATP registers and the Translation PageTables    
//! If other abstractions come up, they should go here. 
//! 
//! Also, Future Me, You got this. Ha ha. 
//! Numb out the pressure, life is useless, just chill out. It's all pointless. And anyway evryting eventually works out...  
//! Or maybe there will come a time when it won't work out. And ... and nothing. It just won't work out. And that's super fine.  
//! THe worst that can happen is ... I don't know... but I doubt it's worth losing shit over.  
//! Let the worst happen. Let the worst happen. So what if you die of hunger?  
//! That's why this project is called Hobo.  
//! We may eventually end up homeless. 
//! Maybe you are already homeless ha ha ha... too much? - OK.  
//! Bye, from the past.
//! 
//! Anyway, this time you were trying to port the byte allocator from Stephen Marz Blog. And you did not undrstand the thing fully. Bugs are haunitng you in your sleep.
//! And it's not like you are finding fixes. 4 days remaining till project is due. You are theoreticaly F***ed. But we have a bigger goal... ad that goal is not academia.
//! No more trauma.  
//! 

use volatile_register::{RO, RW};

// THe SATP register
#[repr(C)]
struct SATP_reg{
    val : RW<u64>
}

impl SATP_reg {
    fn set_mode () { unimplemented!()}
    fn set_ASID () {unimplemented!()}
    fn set_root_addr () {unimplemented!()}
}




// A Table Entry
#[repr(C)]
pub struct TableEntry{
    pub val : u64
}

// setter functions for the flags
// Flags ommited are inconsequential for now
impl TableEntry{
    /// constructor. It sets all the bits of the Table Entry to ZERO
    pub fn new()-> Self{
        TableEntry { val: 0u64 }
    }

    /// sets the entry's value to be the passed address shifted to the right by 2 bits    
    /// It assumes that the address passed has a trailing 12 zeroes, ie divisible by 4096   
    /// It takes care of the required 2 right shifts. Don't worry about that    
    /// It resets all flag bits to ZERO
    pub fn set_address(&mut self, address: u64){
      let new_addr = address >> 2;
      self.val = new_addr;
    }

    /// adds the access bits to the Table entry
    pub fn add_access_mask(&mut self, access_map: u64){
        self.val = self.val | access_map;
    }

    pub fn set_as_valid(&mut self){ self.val = self.val | 1u64;  }
    pub fn set_as_invalid(&mut self){ self.val = self.val & !1u64;  }
    pub fn set_as_readable(&mut self) { self.val = self.val | 2u64;  }
    pub fn set_as_non_readable(&mut self) { self.val = self.val & !2u64;  }
    pub fn set_as_writable(&mut self) { self.val = self.val | 4u64;  }
    pub fn set_as_non_writable(&mut self) { self.val = self.val & !4u64;  }
    pub fn set_as_executable(&mut self) { self.val = self.val | 8u64; }
    pub fn set_as_non_executable(&mut self) { self.val = self.val & !8u64; }
    pub fn set_as_usermode_only(&mut self) { self.val = self.val | 16u64; }
}

// getter funtions
impl TableEntry{
    pub fn get_val(&self) -> u64{   self.val  }

    /// Gets the physical address of the page being refered to. 
    /// The page returned is already shifted to the left by 2 bits 
    pub fn get_address(&self) -> u64 {  
        let val = self.get_val();
        let address = (val << 2) & !0b111111111111; // zero out the last 12 bits
        return address;
    }
}

// checker functions
impl TableEntry{
    pub fn check_if_valid(&self) -> bool{
        if self.val & 1u64 == 1u64 { return true; }
        else { return false;    }
    }

    pub fn check_if_branch(&self) -> bool{
        if self.val & 14u64 == 0 { return true; }
        else {return false; }
    }

    pub fn check_if_leaf(&self) -> bool{
        if self.val & 14u64 != 0 { return true; }
        else {return false; }
    }

    pub fn check_if_readable(&self) -> bool{
        if self.val & 2u64 == 2u64 { true }
        else {  false   }
    }

    pub fn check_if_writable(&self) -> bool{
        if self.val & 4u64 == 4u64 { true }
        else {  false   }
    }

    pub fn check_if_executable(&self) -> bool{
        if self.val & 8u64 == 8u64 { true }
        else {  false   }
    }

    pub fn check_if_usermode_only(&self) -> bool{
        if self.val & 16u64 == 16u64 { true }
        else {  false   }
    }
}




// A Table
// A Page Table has 512 table entries
pub struct Table{
    pub content : [TableEntry; 512]
}


// This table is for demonstration purposes
struct SimpleTranslationTable{
    
}
