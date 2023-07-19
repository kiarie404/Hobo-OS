//! This module outlines functions and macros that take input from the keyboard.  
//! The Keyboard is the standard input.  
//! The UART has been connected to it using the Receive transmission Pin    

// this fuction continuously reads from the UART buffer until the user inputs a new_line character.  
// However, this function can read a maximum of 100 characters, this is because we cannot afford to implement vectors in a no-std enviromrnt    
// The keyboard can only understand ASCII characters
fn read_line() {
    // let mut byte
}



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
                _ => {print!("{}", c as char); }  // print ascii character on screen
            }
        }
    }
}