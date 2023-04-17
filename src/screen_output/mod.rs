// This module presents macros you can use to print to the screen.
// mod uart;
pub mod uart;

use core::write;

#[macro_export]
macro_rules! print {
    // a token is anything : from a costant to a variablle to a struct. Anything
    // print accepts one or more tokens and prints them.... 
    ($($token: tt)+) => ({
        // Uart::new is public
        // Although we re_create the buffer each time, we target the same memory location each time
        use core::fmt::Write;  // remove this to see the error. I am confused about the differences between the Writes
		let _ = write!(crate::screen_output::uart::Uart::new(0x1000_0000), $($token)+); // it's like macro_exports are their own block
        // hen you use macro_export, the macro becomes its own item within the crate, 
        // and it no longer has access to the parent module's private items, including other modules.
        // you need to use an absolute path to reference a module, starting from the crate root. 
        // This ensures that the macro can find the module regardless of where it is used in the crate.
    });
}


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

