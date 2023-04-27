// denounce using the usual entry point chain
#![no_main]   

// this code does not depend on any standard Library. 
// It however depends on the Rust Core Library : https://doc.rust-lang.org/beta/core/index.html
#![no_std]


use core::panic::PanicInfo;


// imported modules section
mod asm;
mod screen_output;
mod keyboard_interface;



#[no_mangle]
pub extern "C" fn kmain() -> (){
    let x :i32 = 20;
    println!("unbuilt, insecure, not_tired , {}", x);
    print_memory_locations();

    keyboard_interface::continuous_keyboard_read();

    println!("unbuilt, insecure, not_tired");
}

fn print_memory_locations(){
    extern "C" {
        static _text_start : usize;
        static _memory_end : usize;
        static _heap_start : usize;
        static _heap_size : usize;
    }

    unsafe{
        // println!("memory start : {:x}", _memory_start);
        println!("memory end : {:x}", _heap_size);
        // println!("memory size : {:x}", (_memory_end - _memory_start));
    }
}

// define the panic handler function for the bare_metal software
// However this &PanicInfo does not contain unwinding information. Traceback -- we have disabled unwinding 
// And anyway it wil not get called upon because panics cause immediate program termination
#[panic_handler]
fn panic_response(_info: &PanicInfo<'_> ) -> !{
    loop {     } // loop indefinately
}

