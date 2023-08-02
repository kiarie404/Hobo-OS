use super::TrapFrame;
use crate::{print, println};

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

pub fn handle_exception(trapframe: &mut TrapFrame){
    // get cause
    let cause = trapframe.mcause & !(1 << 63);

    // match cause
    match cause {
        0   => {
            println!("Handling InstructionAddressMisaligned");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },
        1   => {
            println!("Handling InstructionAccessFault");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },

        2   => {
            println!("Handling IllegalInstruction : {}", trapframe.mepc);
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },

        3   => {
            println!("Handling Breakpoint");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },
        4   => {
            println!("Handling LoadAddressMisaligned");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },
        5   => {
            println!("Handling LoadAccessFault");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },
        6   => {
            println!("Handling StoreAddressMisaligned");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },
        7   => {
            println!("Handling StoreAccessFault");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },
        8   => {
            println!("Handling UserEnvironmentCall");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },
        9   => {
            println!("Handling SupervisorEnvironmentCall");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },
        11   => {
            println!("Handling MachineEnvironmentCall");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },
        12   => {
            println!("Handling InstructionPageFault");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },
        13   => {
            println!("Handling LoadPageFault");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },
        15   => {
            println!("Handling StorePageFault");
            trapframe.mepc += 4; // add 32 bits, so as to point to the istruction that comes after the exception-causing instruction
        },

        _   => {    panic!("Unhandled exception trap cause -> {}\n", cause);  }
    }
}