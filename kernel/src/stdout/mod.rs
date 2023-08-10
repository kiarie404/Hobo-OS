//!  This module provides the functions and macros required to write utf-8 stream of character(s) to the screen.   
//! 
//!  In this case, the the screen is the console.    
//!  The UART device out transmission is connected to the Console


pub static mut STDOUT_BUFFER: [u8; 100] = [27; 100]; // fill the Buffer with esc characters to signiy that it is empty.


/// This macro prints a formatted string to the console.  
/// This macro is callable across the whole crate. It can also be called by external crates
#[macro_export]
macro_rules! print {
    // a token is anything : from a costant to a variablle to a struct. Anything
    // print accepts one or more tokens and prints them.... 
    ($($token: tt)+) => ({
        // Uart::new is public
        // Although we re_create the buffer each time, we target the same memory location each time
        use core::fmt::Write;  // remove this to see t The Rust compiler takes the matched arm and extracts the variable from the argument stringhe error. I am confused about the differences between the Writes
        use crate::drivers::uart::UartDevice;
        let mut uart_instance = UartDevice::new();
		let _ = write!(uart_instance, $($token)+); // it's like macro_exports are their own block
        // when you use macro_export, the macro becomes its own item within the crate, 
        // and it no longer has access to the parent module's private items, including other modules.
        // you need to use an absolute path to reference a module, starting from the crate root. 
        // This ensures that the macro can find the module regardless of where it is used in the crate.
    });
}

/// This macro prints a string literal and adds a new line.  
/// It prints to the standard output. In our case, that's the console
#[macro_export]
macro_rules! println {
    () => ({
		print!("\r\n")
	});
	($fmt_string:expr) => ({
		print!(concat!($fmt_string, "\r\n"))
	});
	($fmt_string:expr, $($args:tt)+) => ({
		print!(concat!($fmt_string, "\r\n"), $($args)+)
	});
}


pub fn flush_std_buffer(){
    let buffer_empty = check_if_buffer_is_empty();
    let buffer_ref = unsafe{&STDOUT_BUFFER};
    if buffer_empty == false{
        let buffer_as_str = core::str::from_utf8(buffer_ref).unwrap();
        println!("{}", buffer_as_str);
    }
}

fn check_if_buffer_is_empty() -> bool{
    // buffer is empty if it is filled with escape_ascii values
    let stdout_buffer_ref = unsafe{&STDOUT_BUFFER};
    if stdout_buffer_ref == &[27; 100]{ return true; }
    else {return false} 
}