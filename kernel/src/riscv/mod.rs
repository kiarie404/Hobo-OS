//! This module contains functions for interacting with a riscv CPU  
//! 
//! 
//! It is a bunch of unsafe inline assembly instructions that have been wrapped with safe Rust code  
//! This is meant to reduce the amount of unsafe blocks in the codebase
//! 

use core::arch::asm;

/// This function reads the value contained in the mstatus register and returns the value
pub fn mstatus_read() -> usize{
    unsafe{
        let value: u64;
        asm!("csrr  {}, mstatus", out(reg) value);
        return value as usize;
    }
}

/// This function reads the value contained in the sstatus register and returns the value
pub fn sstatus_read() -> usize{
    unsafe{
        let value: u64;
        asm!("csrr  {}, sstatus", out(reg) value);
        return value as usize;
    }
}

/// This function reads the value contained in the mscratch register and returns the value
pub fn mscratch_read() -> usize{
    unsafe{
        let value: u64;
        asm!("csrr  {}, mscratch", out(reg) value);
        return value as usize;
    }
}

/// This function reads the value contained in the sscratch register and returns the value
pub fn sscratch_read() -> usize{
    unsafe{
        let value: u64;
        asm!("csrr  {}, sscratch", out(reg) value);
        return value as usize;
    }
}

/// This function reads the value contained in the mepc register and returns the value
pub fn mepc_read() -> usize{
    unsafe{
        let value: u64;
        asm!("csrr  {}, mepc", out(reg) value);
        return value as usize;
    }
}

/// This function reads the value contained in the sepc register and returns the value
pub fn sepc_read() -> usize{
    unsafe{
        let value: u64;
        asm!("csrr  {}, sepc", out(reg) value);
        return value as usize;
    }
}

/// This function reads the value contained in the satp register and returns the value
pub fn satp_read() -> usize{
    unsafe{
        let value: u64;
        asm!("csrr  {}, satp", out(reg) value);
        return value as usize;
    }
}

/// This function reads the value contained in the stvec register and returns the value
pub fn stvec_read() -> usize{
    unsafe{
        let value: u64;
        asm!("csrr  {}, stvec", out(reg) value);
        return value as usize;
    }
}

/// This function reads the value contained in the mtvec register and returns the value
pub fn mtvec_read() -> usize{
    unsafe{
        let value: u64;
        asm!("csrr  {}, mtvec", out(reg) value);
        return value as usize;
    }
}

/// This function reads the value contained in the mcause register and returns the value
pub fn mcause_read() -> usize{
    unsafe{
        let value: u64;
        asm!("csrr  {}, mcause", out(reg) value);
        return value as usize;
    }
}

/// This function reads the value contained in the mie register and returns the value
pub fn mie_read() -> usize{
    unsafe{
        let value: u64;
        asm!("csrr  {}, mie", out(reg) value);
        return value as usize;
    }
}

/// This function reads the value contained in the mtval register and returns the value
pub fn mtval_read() -> usize{
    unsafe{
        let value: u64;
        asm!("csrr  {}, mtval", out(reg) value);
        return value as usize;
    }
}

// ------------ Writing Functions ----------------------- //
pub fn mscratch_write(val: u64){
    unsafe{
        asm!("csrw  mscratch, {}", in(reg) val);
    }
}

pub fn sscratch_write(val: u64){
    unsafe{
        asm!("csrw  sscratch, {}", in(reg) val);
    }
}

pub fn stvec_write(val: u64){
    unsafe{
        asm!("csrw  stvec, {}", in(reg) val);
    }
}

pub fn mtvec_write(val: u64){
    unsafe{
        asm!("csrw  mtvec, {}", in(reg) val);
    }
}

pub fn satp_write(val: u64){
    unsafe{
        asm!("csrw  satp, {}", in(reg) val);
    }
}

pub fn mepc_write(val: u64){
    unsafe{
        asm!("csrw  mepc, {}", in(reg) val);
    }
}

pub fn sepc_write(val: u64){
    unsafe{
        asm!("csrw  sepc, {}", in(reg) val);
    }
}


pub fn clear_TLB(){
    unsafe{
        asm!("sfence.vma");
    }
}


// ----------- control functions -------------------------------- //
pub fn call_mret(){
    unsafe {asm!("mret")};
}

pub fn cpu_shutdown(){
    unsafe{
        loop{
            asm!("wfi");
        }
    }
}