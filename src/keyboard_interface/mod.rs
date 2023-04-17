use crate::{print, println};

mod uart;

pub fn continuous_keyboard_read (){
    let mut uart_instance = uart::Uart::new(0x1000_0000);
    uart_instance.init();
    loop {
        if let Some(c) = uart_instance.read_uart_buffer() {
            match c {
                10 | 13 => { println!(); },
                27      => { break; }
                _ => {print!("{}", c as char); }
            }
        }
    }
}