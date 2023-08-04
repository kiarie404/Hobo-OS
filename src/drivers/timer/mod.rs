//! This module abstracts the CLINT Core Local Interruptor Timer    
//! It exposes two registers : the mtime and mtimecmp.  
//! It also exposes time-setting functions


// Constants according to Qemu
const MTIME_ADDRESS: usize = 0x0200_bff8;
const MTIMECMP_ADDRESS: usize = 0x02004000;

const MTIME_PTR: *mut usize = MTIME_ADDRESS as *mut usize;
const MTIMECMP_PTR: *mut usize = MTIMECMP_ADDRESS as *mut usize;



pub struct Timer{ /* unit struct... This was a blanket for the API */}

impl Timer{
    pub fn mtime_read() -> usize{
        unsafe { MTIME_PTR.read_volatile() }
    }

    pub fn mtimecmp_read() -> usize{
        unsafe {    MTIMECMP_PTR.read_volatile() }
    }

    pub fn mtimecmp_write(value: usize){
        unsafe { MTIMECMP_PTR.write_volatile(value);    }
    }
}