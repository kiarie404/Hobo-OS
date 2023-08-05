//! THis module abstracts the PLIC (Platform Level Interrup Controller)
//! It presents functions that can iteract with the underlying PLIC that cervices HART 0 Only
//! 

mod errors;

use self::errors::PlicError;

// PLIC register addresses
const PLIC_PRIORITY: usize = 0x0c00_0000;
const PLIC_PENDING: usize = 0x0c00_1000;
const PLIC_INT_ENABLE: usize = 0x0c00_2000;
const PLIC_THRESHOLD: usize = 0x0c20_0000;
const PLIC_BUFFER: usize = 0x0c20_0004;




/// This function reads the Interrupt ID value found in the buffer register
pub fn read_ID_from_buffer() -> Option<u32>{
    let ptr = PLIC_BUFFER as *const u32;
    let value =  unsafe {ptr.read_volatile()};

    if value == 0{ return None; }
    else {  return Some(value); }
}

/// THis function writes the interrupt ID to the Buffer in order to 
/// notify the PLIC that the Interrupt has already been handled by the CPU
pub fn write_ID_to_buffer(interrupt_id: u32) -> Result<(), PlicError>{
    let ptr = PLIC_BUFFER as *mut u32;

    if interrupt_id == 0{
        return Err(errors::PLIC_ERROR_Invalid_Interrupt_ID);
    }
    unsafe {ptr.write_volatile(interrupt_id)};
    return Ok(());
}

/// Sets the value of the threshold Register
pub fn threshold_write( limit: u8) -> Result<(), PlicError >{
    if limit < 0 || limit > 7 { return Err(errors::PLIC_ERROR_Invalid_Threshold_Value);  }
    let ptr = PLIC_THRESHOLD as *mut u32;
    unsafe {ptr.write_volatile(limit as u32)};
    Ok(())
}

/// Reads the threshold Register
pub fn threshold_read() -> u8{
    let ptr = PLIC_THRESHOLD as *const u32;
    let value = unsafe { ptr.read_volatile()};
    return value as u8;
}

/// Enables the Interrupt associated with the input Interrupt ID
pub fn enable_interrupt(interrupt_id: u32){
    let ptr = PLIC_INT_ENABLE as *mut u32;
    let actual_id = 1 << interrupt_id;
    unsafe {
        ptr.write_volatile(ptr.read_volatile() | actual_id);
    }
}

/// Disables the Interrupt associated with the input Interrupt ID
// pub fn disable_interrupt(interrupt_id: u32) -> Result<(), PlicError>{
//     unimplemented!()
// }


/// Sets the priority value of the associated interrupt
pub fn priority_write(interrupt_id: u32, priority_value: u8) -> Result<(), PlicError>{
    if priority_value < 0 || priority_value > 7 {   return Err(errors::PLIC_ERROR_Invalid_Priority_Value);}

    let ptr = PLIC_PRIORITY as *mut u32;
    unsafe {ptr.add(interrupt_id as usize).write_volatile(priority_value as u32);}
    Ok(())
}

/// Reads the priority value of the associated interrupt
pub fn priority_read(interrupt_id: u32) -> u8{
    let ptr = PLIC_PRIORITY as *const u32;
    let value = unsafe { ptr.add(interrupt_id as usize).read_volatile()};
    return value as u8;
}

/// See if a given interrupt id is pending.
pub fn is_pending(id: u32) -> bool {
    let pend = PLIC_PENDING as *const u32;
    let actual_id = 1 << id;
    let pend_ids;
    unsafe {
        pend_ids = pend.read_volatile();
    }
    actual_id & pend_ids != 0
}



// // THis function returns an array of all pending interrupts
// pub fn get_pending_interrupts() ->  Result<(), PlicError >{
//     unimplemented!()
// } 

// /// THis function returns an array of all enabled interrupts
// pub fn get_enabled_interrupts() ->  Result<(), PlicError >{
//     unimplemented!()
// } 

// /// THis function returns an array of all disabled interrupts
// pub fn get_disabled_interrupts() ->  Result<(), PlicError >{
//     unimplemented!()
// } 



