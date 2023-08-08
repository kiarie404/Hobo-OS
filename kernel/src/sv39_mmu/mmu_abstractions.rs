//! Here are the abstractions of the MMU related hardware   
//! More specifically, the SATP registers and the Translation PageTables    
//! If other abstractions come up, they should go here
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

    pub fn set_val_with_access_map(&mut self, access_map: u64){
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
