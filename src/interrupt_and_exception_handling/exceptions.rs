use super::TrapFrame;
use crate::{print, println};
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

pub fn handle_exception(trapframe: &mut TrapFrame) -> usize{
    // get cause
    let cause = trapframe.mcause & !(1 << 63);

    // match cause
    match cause {
        0   => {
            println!("Handling InstructionAddressMisaligned");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },
        1   => {
            println!("Handling InstructionAccessFault");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },

        2   => {
            println!("Handling IllegalInstruction : {}", trapframe.mepc);
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },

        3   => {
            println!("Handling Breakpoint");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },
        4   => {
            println!("Handling LoadAddressMisaligned");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },
        5   => {
            println!("Handling LoadAccessFault");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },
        6   => {
            println!("Handling StoreAddressMisaligned");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },
        7   => {
            println!("Handling StoreAccessFault");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },
        8   => {
            println!("Handling UserEnvironmentCall");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },
        9   => {
            println!("Handling SupervisorEnvironmentCall");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },
        11   => {
            println!("Handling MachineEnvironmentCall");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },
        12   => {
            println!("Handling InstructionPageFault");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },
        13   => {
            println!("Handling LoadPageFault");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },
        15   => {
            println!("Handling StorePageFault");
            let next_doable_instruction = trapframe.mepc + 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
            return next_doable_instruction;
        },

        _   => {    panic!("Unhandled exception trap cause -> {}\n", cause);  }
    }
}