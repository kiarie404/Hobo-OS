use super::TrapFrame;
use crate::{print, println};

/// Interrupt enumeration
#[derive(Debug, Clone, Copy)]
pub enum InterruptType
{
    UserSoftwareInterrupt, // 0
    SupervisorSoftwareInterrupt, // 1
    MachineSoftwareInterrupt, // 3
    UserTimerInterrupt, // 4
    SupervisorTimerInterrupt, // 5
    MachineTimerInterrupt, // 7
    UserExternalInterrupt, // 8
    SupervisorExternalInterrupt, // 9
    MachineExternalInterrupt, // 11
    UnknownAsync(usize)// >= 16  
}

pub fn handle_interrupt(trapframe: &mut TrapFrame){
    // get interrupt cause
    let cause = trapframe.mcause & !(1<<63);

    // match interrupt
    match cause{
        0 => {
            println!(" Handling UserSoftwareInterrupt");
        }
        1 => {
            println!(" Handling SupervisorSoftwareInterrupt");
        }
        3 => {
            println!(" Handling MachineSoftwareInterrupt");
        }
        4 => {
            println!(" Handling UserTimerInterrupt");
        }
        5 => {
            println!(" Handling SupervisorTimerInterrupt");
        }
        7 => {
            println!(" Handling MachineTimerInterrupt");
        }
        8 => {
            println!(" Handling UserExternalInterrupt");
        }
        9 => {
            println!(" Handling SupervisorExternalInterrupt");
        }
        11 => {
            println!(" Handling MachineExternalInterrupt");
        }
        _ => {
            println!(" Captured an undefined Interrupt");
        }
    }
}

