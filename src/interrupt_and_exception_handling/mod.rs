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
    pub satp : usize,
    pub mstatus : usize,
    pub mepc : usize,
    pub mie : usize,
    pub mcause : usize,
    pub mtval : usize,
    // trap stack 
    pub trap_stack : [usize; 10]
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
pub extern "C" fn rust_trap(){

    println!("I am in rust trap handler");
    let trap_frame_ref = unsafe{ &mut kernel_trap_frame};

    // save the regs in the trapframe
    save_integer_registers( trap_frame_ref);
    save_csr_registers( trap_frame_ref);

    // change the stack pointer to point to the floor of the trapstack
    let trap_stack_floor_memory_ptr = (&mut trap_frame_ref.trap_stack[9]) as *mut usize;
    let trap_stack_floor_memory_address = trap_stack_floor_memory_ptr as usize; 
    unsafe { asm!("mv  sp, {}", in(reg)trap_stack_floor_memory_address)};


    // check if disturbance was an exception or interrupt
    let was_interrupt = check_if_interrupt(trap_frame_ref);

    // Handle the exception or interrupt
    if was_interrupt == true {    interrupts::handle_interrupt(trap_frame_ref);    }
    else{    exceptions::handle_exception(trap_frame_ref);    }

    // restore the values of the trapframe... before calling mret
    // restore_integer_registers(trap_frame_ref);
    // restore_csr_registers(trap_frame_ref);

    // call mret. Return to the process that caused a fuss
    // riscv::call_mret();


}

/// This function reads the mcause register found in the trapframe and 
/// determines if the cause of trap was an interrupt or Exception.  
/// Th function returns true if the occurence was an interrupt and false if the occurence was an exception
fn check_if_interrupt(trapframe: &TrapFrame) -> bool{
    let mcause_value = trapframe.satp;
    // extract async/sync bit. ie bit[63]
    let async_sync_bit = mcause_value >> 63;

    if async_sync_bit == 1 {    return true;    }
    else {  return false;   }
}

/// This function reads the mcause register found in the trapframe and 
/// determines if the cause of trap was an Exception.  
/// The function returns true if the occurence was an exception and false if the occurence was an interrupt
fn check_if_exception(trapframe: &TrapFrame) -> bool{
    let mcause_value = trapframe.satp;
    // extract async/sync bit. ie bit[63]
    let async_sync_bit = mcause_value >> 63;

    if async_sync_bit == 0 {    return true;    }
    else {  return false;   }
}



fn save_csr_registers(trapframe: &mut TrapFrame){
    trapframe.satp = riscv::satp_read();
    trapframe.mcause = riscv::mcause_read();
    trapframe.mepc = riscv::mepc_read();
    trapframe.mie = riscv::mie_read();
    trapframe.mstatus = riscv::mstatus_read();
    trapframe.mtval = riscv::mtval_read();
}

fn restore_csr_registers(trapframe: &mut TrapFrame){
    // these two values are the ones that are relevant, theymight hae changed
    riscv::satp_write(trapframe.satp as u64);
    riscv::mepc_write(trapframe.mepc as u64);

    // we do not restore the mstatus, and mie because that will get done...
    // ... automatically when we call mret
}


fn save_integer_registers(trapframe: &mut TrapFrame){

    // I will eventually use a macro. But for now, this will do :
    // it is just 32 lines... a disgrace to loops and macros
    unsafe{
        asm!("mv  {}, x0", out(reg) trapframe.regs[0]);
        asm!("mv  {}, x1", out(reg) trapframe.regs[1]);
        asm!("mv  {}, x2", out(reg) trapframe.regs[2]);
        asm!("mv  {}, x3", out(reg) trapframe.regs[3]);
        asm!("mv  {}, x4", out(reg) trapframe.regs[4]);
        asm!("mv  {}, x5", out(reg) trapframe.regs[5]);
        asm!("mv  {}, x6", out(reg) trapframe.regs[6]);
        asm!("mv  {}, x7", out(reg) trapframe.regs[7]);
        asm!("mv  {}, x8", out(reg) trapframe.regs[8]);
        asm!("mv  {}, x9", out(reg) trapframe.regs[9]);
        asm!("mv  {}, x10", out(reg) trapframe.regs[10]);
        asm!("mv  {}, x11", out(reg) trapframe.regs[11]);
        asm!("mv  {}, x12", out(reg) trapframe.regs[12]);
        asm!("mv  {}, x13", out(reg) trapframe.regs[13]);
        asm!("mv  {}, x14", out(reg) trapframe.regs[14]);
        asm!("mv  {}, x15", out(reg) trapframe.regs[15]);
        asm!("mv  {}, x16", out(reg) trapframe.regs[16]);
        asm!("mv  {}, x17", out(reg) trapframe.regs[17]);
        asm!("mv  {}, x18", out(reg) trapframe.regs[18]);
        asm!("mv  {}, x19", out(reg) trapframe.regs[19]);
        asm!("mv  {}, x20", out(reg) trapframe.regs[20]);
        asm!("mv  {}, x21", out(reg) trapframe.regs[21]);
        asm!("mv  {}, x22", out(reg) trapframe.regs[22]);
        asm!("mv  {}, x23", out(reg) trapframe.regs[23]);
        asm!("mv  {}, x24", out(reg) trapframe.regs[24]);
        asm!("mv  {}, x25", out(reg) trapframe.regs[25]);
        asm!("mv  {}, x26", out(reg) trapframe.regs[26]);
        asm!("mv  {}, x27", out(reg) trapframe.regs[27]);
        asm!("mv  {}, x28", out(reg) trapframe.regs[28]);
        asm!("mv  {}, x29", out(reg) trapframe.regs[29]);
        asm!("mv  {}, x30", out(reg) trapframe.regs[30]);
        asm!("mv  {}, x31", out(reg) trapframe.regs[31]);
    }

}

fn restore_integer_registers(trapframe: &mut TrapFrame){

    // I will eventually use a macro. But for now, this will do :
    // it is just 32 lines... a disgrace to loops and macros
    unsafe{
        asm!("mv  x0, {}", in(reg) trapframe.regs[0]);
        asm!("mv  x1, {}", in(reg) trapframe.regs[1]);
        asm!("mv  x2, {}", in(reg) trapframe.regs[2]);
        asm!("mv  x3, {}", in(reg) trapframe.regs[3]);
        asm!("mv  x4, {}", in(reg) trapframe.regs[4]);
        asm!("mv  x5, {}", in(reg) trapframe.regs[5]);
        asm!("mv  x6, {}", in(reg) trapframe.regs[6]);
        asm!("mv  x7, {}", in(reg) trapframe.regs[7]);
        asm!("mv  x8, {}", in(reg) trapframe.regs[8]);
        asm!("mv  x9, {}", in(reg) trapframe.regs[9]);
        asm!("mv  x10, {}", in(reg) trapframe.regs[10]);
        asm!("mv  x11, {}", in(reg) trapframe.regs[11]);
        asm!("mv  x12, {}", in(reg) trapframe.regs[12]);
        asm!("mv  x13, {}", in(reg) trapframe.regs[13]);
        asm!("mv  x14, {}", in(reg) trapframe.regs[14]);
        asm!("mv  x15, {}", in(reg) trapframe.regs[15]);
        asm!("mv  x16, {}", in(reg) trapframe.regs[16]);
        asm!("mv  x17, {}", in(reg) trapframe.regs[17]);
        asm!("mv  x18, {}", in(reg) trapframe.regs[18]);
        asm!("mv  x19, {}", in(reg) trapframe.regs[19]);
        asm!("mv  x20, {}", in(reg) trapframe.regs[20]);
        asm!("mv  x21, {}", in(reg) trapframe.regs[21]);
        asm!("mv  x22, {}", in(reg) trapframe.regs[22]);
        asm!("mv  x23, {}", in(reg) trapframe.regs[23]);
        asm!("mv  x24, {}", in(reg) trapframe.regs[24]);
        asm!("mv  x25, {}", in(reg) trapframe.regs[25]);
        asm!("mv  x26, {}", in(reg) trapframe.regs[26]);
        asm!("mv  x27, {}", in(reg) trapframe.regs[27]);
        asm!("mv  x28, {}", in(reg) trapframe.regs[28]);
        asm!("mv  x29, {}", in(reg) trapframe.regs[29]);
        asm!("mv  x30, {}", in(reg) trapframe.regs[30]);
        asm!("mv  x31, {}", in(reg) trapframe.regs[31]);
    }

}
