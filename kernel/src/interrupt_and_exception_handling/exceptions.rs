use super::TrapFrame;
use crate::{print, println};
use core::arch::asm;
use crate::riscv;

#[derive(Debug, Clone, Copy)]
pub enum ExceptionType{
    InstructionAddressMisaligned, // 0
    InstructionAccessFault, // 1
    IllegalInstruction, // 2
    Breakpoint, // 3
    LoadAddressMisaligned, // 4
    LoadAccessFault, // 5
    StoreAddressMisaligned, // 6
    StoreAccessFault, // 7
    UserEnvironmentCall, // 8
    SupervisorEnvironmentCall, // 9
    MachineEnvironmentCall, // 11
    InstructionPageFault, // 12
    LoadPageFault, // 13
    StorePageFault, // 15
    UnknownSync(usize), // unassigned/ custom
}

#[derive(Debug, Clone, Copy)]
pub enum ExceptionHandlingError<'a>{
    UnableToRecoverFromException(&'a str),
}

pub fn handle_exception(trapframe: &mut TrapFrame) -> Result<usize, ExceptionHandlingError>{
    // get cause
    let cause = trapframe.mcause & !(1 << 63);

    // match cause
    // todo!("make the error message display the address that caused problems") 
    match cause {
        0   => {
            println!("Handling InstructionAddressMisaligned");
            let misaligned_address = riscv::mtval_read();
            return Err(ExceptionHandlingError::UnableToRecoverFromException("Instruction Address Misaligned Excption occured ")); 
            
        },
        1   => {
            println!("Handling InstructionAccessFault");
            return Err(ExceptionHandlingError::UnableToRecoverFromException("InstructionAccessFault occured "));  
        },

        2   => {
            println!("Handling IllegalInstruction : {}", trapframe.mepc);
            return Err(ExceptionHandlingError::UnableToRecoverFromException("IllegalInstruction occured "));
        },

        3   => {
            println!("Handling Breakpoint");
            let next_nonfaulty_instruction = trapframe.mepc + 4; // $ra + 4
            return Ok(next_nonfaulty_instruction);
        },
        4   => {
            println!("Handling LoadAddressMisaligned");
            return Err(ExceptionHandlingError::UnableToRecoverFromException("LoadAddressMisaligned occured ")); 
        },
        5   => {
            println!("Handling LoadAccessFault");
            return Err(ExceptionHandlingError::UnableToRecoverFromException("LoadAccessFault occured ")); 
        },
        6   => {
            println!("Handling StoreAddressMisaligned");
            return Err(ExceptionHandlingError::UnableToRecoverFromException("StoreAddressMisaligned occured "));
        },
        7   => {
            println!("Handling StoreAccessFault");
            return Err(ExceptionHandlingError::UnableToRecoverFromException("StoreAccessFault occured "));
        },
        8   => {
            println!("Handling UserEnvironmentCall");
            let next_nonfaulty_instruction = trapframe.regs[1] + 4; // $ra + 4
            return Ok(next_nonfaulty_instruction);
        },
        9   => {
            println!("Handling SupervisorEnvironmentCall");
            let next_nonfaulty_instruction = trapframe.regs[1] + 4; // $ra + 4
            return Ok(next_nonfaulty_instruction);
        },
        11   => {
            println!("Handling MachineEnvironmentCall");
            return Err(ExceptionHandlingError::UnableToRecoverFromException("MachineEnvironmentCall occured "));
        },
        12   => {
            println!("Handling InstructionPageFault");
            return Err(ExceptionHandlingError::UnableToRecoverFromException("InstructionPageFault occured "));
        },
        13   => {
            println!("Handling LoadPageFault");
            return Err(ExceptionHandlingError::UnableToRecoverFromException("LoadPageFault occured "));
        },
        15   => {
            println!("Handling StorePageFault");
            return Err(ExceptionHandlingError::UnableToRecoverFromException("StorePageFault occured "));
        },

        _   => {    panic!("Unhandled exception trap cause -> {}\n", cause);  }
    }
}


