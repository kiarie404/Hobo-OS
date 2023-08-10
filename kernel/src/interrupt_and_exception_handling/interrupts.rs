use super::TrapFrame;
use crate::drivers::{plic, uart, self};
use crate::{print, println};
use crate::drivers::timer::Timer;
use crate::{stdout, stdin};

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
             Timer::mtimecmp_write(Timer::mtime_read() + 10_000_000);
        }
        7 => {
            println!(" Handling MachineTimerInterrupt");
            Timer::mtimecmp_write(Timer::mtime_read() + 10_000_000);
        }
        8 => {
            println!(" Handling UserExternalInterrupt");
            sort_external_interrupts();
        }
        9 => {
            println!(" Handling SupervisorExternalInterrupt");
            sort_external_interrupts();
        }
        11 => {
            println!(" Handling MachineExternalInterrupt");
            sort_external_interrupts();
        }
        _ => {
            println!(" Captured an undefined Interrupt");
        }
    }
}

fn sort_external_interrupts(){
    // contact plic and determine which device has sent an interrupt
    let interrupt_ID = plic::read_ID_from_buffer().expect("unable to read plic buffer");

    // sort interrupt
    match interrupt_ID {
        1..=8 => drivers::handle_interrupt(interrupt_ID),
        10 => handle_UART_interrupts(),
        _  => panic!("Received an unknown external Interrupt ID")
    }

    // Notify plic that handling is done
    let write_result = plic::write_ID_to_buffer(interrupt_ID);
    match write_result{
        Ok(()) => {/* Do nothing */},
        Err(passed_err) => println!("{}", passed_err)
    }

}

fn handle_UART_interrupts(){
    // determine which UART interrupt it was
    let uart_instance = uart::UartDevice::new();

    let interrupt_status_reg = uart_instance.read_interrupt_status_reg();
    let masked_reg_value = interrupt_status_reg & 0b0000_1111;
    match masked_reg_value {
        0b0000_0010 => handle_UART_THR_empty_interrupt(),
        0b0000_0100 => handle_UART_Data_Ready(),
        0b0000_1100 => handle_UART_Character_Timeout(),
        _ => panic!("Unhandled UART interrupt value")
    }
}

fn handle_UART_THR_empty_interrupt(){
    stdout::flush_std_buffer();
    print!("c");
}

fn handle_UART_Data_Ready(){
    let input = stdin::read_line().expect("read line failed");
    // println!("{}", input);
}

fn handle_UART_Character_Timeout(){
    let input = stdin::read_line().expect("read line failed");
    // println!("{}", input);`
}



