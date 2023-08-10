//! This module outlines functions and macros that take input from the keyboard.  
//! 
//! 
//! The Keyboard is the standard input.  
//! The UART has been connected to it using the Receive transmission Pin  
//! Reading from the Keyboard is interrupt driven. When the UART receives any input input from the keyboard, it sends an interrupt to the PLIC.
//! THe PLIC in turns calls the CPU to execut a trap.  
//! The keyboard can only understand ASCII characters

use crate::drivers::uart::UartDevice;
use crate::drivers::plic;
use crate::{print,println};


const UART_INTERRUPT_ID: u32 = 10; 
const ARR_SIZE: usize = 100;
pub static mut STDIN_BUFFER: [u8; 100] = [27; 100]; // we could have used a vector... but that will come in the future; fill the buffer with escape character

/// This function prepares the environment for reading from the UART  
/// It does this by :  
/// 1. clearing any garbage values in the UART buffer. Values that may have been received before interrupts were activated
/// 2. Activates UART interrupts in th PLIC
fn initialize_read_environment(){
    // let UART_device = UartDevice::init(); // this does not create a new instance, it just creates a new reference in the background
    // UART_device.empty_FIFO_receive_buffer(); // Our stdin::read_line() needs new clean data

    // clear the STDIN buffer
    let stdin_buffer_ref = unsafe {&mut STDIN_BUFFER};
    for index in 0..stdin_buffer_ref.len(){
        stdin_buffer_ref[index] = 27; // fill buffer with escape characters
    }


    // plic::enable_interrupt(UART_INTERRUPT_ID); // enable interrupts from the UART
    // plic::threshold_write(7).unwrap();
    // plic::priority_write(10, 6).unwrap();



    // so now if the user user types to the keyboard, an interrupt will be sent to the CPU via the PLIC
    // The CPU will call the interrupt handler. 
    // The interrupt handler

    // println!("Environment initialized for stdin");  
    // println!("UART : {}", UART_device);  
    // plic::display();
}

/// This fuction continuously reads from the UART buffer until the user inputs a new_line character.  
/// It then returns a &'static str that is a copy of the stdin array of characters    
/// However, this function can read a maximum of 100 characters, this is because we cannot afford to implement vectors in a no-std enviromrnt    
pub fn read_line()-> Option<[u8; ARR_SIZE]>{
    initialize_read_environment();
    let Uart_device = UartDevice::new(); // new ref
    let mut arr_index = 0; // index to the stdin array
    let stdin_buffer_ref = unsafe {&mut STDIN_BUFFER}; // making the stdin buffer local, reduces unsafe operations
    
    loop {
        let data_option = Uart_device.read_from_buffer();
        match data_option {
            Some(data) => {
                stdin_buffer_ref[arr_index] = data; // store data in the stdin array
                arr_index += 1;
                print!("{}", data as char);
                if data == 10 || data == 13 { // if that byte is a new-line or jump character...
                    break; // ... then stop reading from the UART
                }
            },
            None => {  } // you have read all data, nothing else to do
        }
    }

    // get slice with valid data; 
        let mut first_invalid_index = 0;  // an invalid index points to ascii character 27
        for index in 0..stdin_buffer_ref.len(){
            if stdin_buffer_ref[index] == 27 {     break;    }
            else{ first_invalid_index = index }
        }

    if first_invalid_index == 0 {   return None; } // means there is no data in buffer
    else {
        let valid_slice: [u8; ARR_SIZE] = unsafe {STDIN_BUFFER.clone()};
        return Some(valid_slice);
    }

}

// fn convert_chars_to_str


/// This function continuously reads keybuard input while displaying the inputs on screen (real time)   
/// Pressing the Escape Key breaks this continuous read_and_display
pub fn continuous_keyboard_read (){
    use crate::println;
    use crate::print;
    use crate::drivers::uart::UartDevice;
    let mut uart_instance = UartDevice::new();

    // let character : u8 = uart_instance.read_from_buffer().expect("unable to read from buffer");
    // println!("{}", character);
    loop {
        if let Some(c) = uart_instance.read_from_buffer() {
            match c {
                10 | 13 => { println!(); },  // if new-line key, jump a line
                27      => { break; }        // if escape key is pressed, stop taking input
                _ => {/*print!("{}", c as char); */}  // print ascii character on screen
            }
        }
    }
}