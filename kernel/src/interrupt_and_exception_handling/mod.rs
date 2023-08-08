mod exceptions;
mod interrupts;

use crate::{print, println, riscv};
use crate::kernel_trap_frame;
use core::arch::asm;


/// The trapframe save the context of the CPU when the process that was cut-short was running

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TrapFrame{
    // normal registers
    pub regs : [usize; 32],
    pub fregs: [usize; 32],
    // control status registers 
    pub satp : usize,  // 512
    pub mstatus : usize, // 520
    pub mepc : usize, // 528
    pub mie : usize, // 536
    pub mcause : usize, // 544
    pub mtval : usize, // 552
    // trap stack 
    pub trap_stack : [usize; 10] // [560 - 567]
}

impl TrapFrame {
	pub const fn zero() -> Self {
		TrapFrame { regs:       [0; 32],
		            fregs:      [0; 32],
		            satp:       0,
		            mstatus:    0,
                    mepc:    0,
                    mie:    0,
                    mcause:    0,
                    mtval:    0,
                    trap_stack:    [0; 10],
                 }
   }
}


#[no_mangle]
pub extern "C" fn rust_trap_handler()-> usize{

    println!("I am in rust trap handler");
    let trap_frame_ref = unsafe{ &mut kernel_trap_frame};


    // change the stack pointer to point to the floor of the trapstack
    // let trap_stack_floor_memory_ptr = (&mut trap_frame_ref.trap_stack[9]) as *mut usize;
    // let trap_stack_floor_memory_address = trap_stack_floor_memory_ptr as usize; 
    // unsafe { asm!("mv  sp, {}", in(reg)trap_stack_floor_memory_address)};


    // check if disturbance was an exception or interrupt
    let was_interrupt = check_if_interrupt(trap_frame_ref);

    // Handle the exception or interrupt
    if was_interrupt == true {    
        interrupts::handle_interrupt(trap_frame_ref); 
        return trap_frame_ref.mepc;
       }
    else{   
         let exception_handling_result = exceptions::handle_exception(trap_frame_ref);
         match exception_handling_result {
            Ok(address) => {    return address;  },
            Err(exception_handling_error) => {
                println!("Exeption Handling Error Occurred : {:?}", exception_handling_error);
                println!("Kernel will shut down");
                riscv::cpu_shutdown();
                return 0;
            }
         }
    }


}

/// This function reads the mcause register found in the trapframe and 
/// determines if the cause of trap was an interrupt or Exception.  
/// Th function returns true if the occurence was an interrupt and false if the occurence was an exception
fn check_if_interrupt(trapframe: &TrapFrame) -> bool{
    let mcause_value = trapframe.mcause;
    // extract async/sync bit. ie bit[63]
    let async_sync_bit = mcause_value >> 63;

    if async_sync_bit == 1 {    return true;    }
    else {  return false;   }
}

/// This function reads the mcause register found in the trapframe and 
/// determines if the cause of trap was an Exception.  
/// The function returns true if the occurence was an exception and false if the occurence was an interrupt
fn check_if_exception(trapframe: &TrapFrame) -> bool{
    let mcause_value = trapframe.mcause;
    // extract async/sync bit. ie bit[63]
    let async_sync_bit = mcause_value >> 63;

    if async_sync_bit == 0 {    return true;    }
    else {  return false;   }
}


pub fn init_kernel_trap_handling(){
    // store kernel trapframe into the mscratch register
    let kernel_trapframe_ref = unsafe{&crate::kernel_trap_frame};
    let kernel_trapframe_ptr = kernel_trapframe_ref as *const TrapFrame;
    let kernel_trapframe_address = kernel_trapframe_ptr as u64;
    riscv::mscratch_write(kernel_trapframe_address);
}
