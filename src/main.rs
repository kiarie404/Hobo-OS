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


// shenanigans
// Constants from the linker script

// Constants recieved from the linker script
extern "C"
{
    static TEXT_START: usize;
    static TEXT_END: usize;
    static RODATA_START: usize;
    static RODATA_END: usize;
    static DATA_START: usize;
    static DATA_END: usize;
    static BSS_START: usize;
    static BSS_END: usize;
    static KERNEL_STACK_START: usize;
    static KERNEL_STACK_END: usize;
    static HEAP_START: usize;
    static HEAP_END: usize;
}

/// Get the text start address as a usize
/// Safety: Because this value should have been read properly from the linker
/// script, this is safe
pub fn text_start() -> usize
{
	unsafe { TEXT_START }
}

/// Get the text end address as a usize
/// Safety: Because this value should have been read properly from the linker
/// script, this is safe
pub fn text_end() -> usize
{
	unsafe { TEXT_END }
}

/// Get the rodata start address as a usize
/// Safety: Because this value should have been read properly from the linker
/// script, this is safe
pub fn rodata_start() -> usize
{
	unsafe { RODATA_START }
}

/// Get the rodata end address as a usize
/// Safety: Because this value should have been read properly from the linker
/// script, this is safe
pub fn rodata_end() -> usize
{
	unsafe { RODATA_END }
}

/// Get the data start address as a usize
/// Safety: Because this value should have been read properly from the linker
/// script, this is safe
pub fn data_start() -> usize
{
	unsafe { DATA_START }
}

/// Get the data end address as a usize
/// Safety: Because this value should have been read properly from the linker
/// script, this is safe
pub fn data_end() -> usize
{
	unsafe { DATA_END }
}

/// Get the bss start address as a usize
/// Safety: Because this value should have been read properly from the linker
/// script, this is safe
pub fn bss_start() -> usize
{
	unsafe { BSS_START }
}

/// Get the bss end address as a usize
/// Safety: Because this value should have been read properly from the linker
/// script, this is safe
pub fn bss_end() -> usize
{
	unsafe { BSS_END }
}

/// Get the stack start address as a usize
/// Safety: Because this value should have been read properly from the linker
/// script, this is safe
pub fn stack_start() -> usize
{
	unsafe { KERNEL_STACK_START }
}

/// Get the stack end address as a usize
/// Safety: Because this value should have been read properly from the linker
/// script, this is safe
pub fn stack_end() -> usize
{
	unsafe { KERNEL_STACK_END }
}

/// Get the heap start address as a usize
/// Safety: Because this value should have been read properly from the linker
/// script, this is safe
pub fn heap_start() -> usize
{
	unsafe { HEAP_START }
}

/// Get the heap end address as a usize
/// Safety: Because this value should have been read properly from the linker
/// script, this is safe
pub fn heap_end() -> usize
{
	unsafe { HEAP_END }
}

// end of shenanigans



#[no_mangle]
pub extern "C" fn kmain() -> (){
    let x :i32 = 20;
    println!("unbuilt, insecure, not_tired , {}", x);

    let start = text_start();
    println!("Memory start : {:x}", start);
    unsafe {println!("Memory start : {:x}", HEAP_START);}

    keyboard_interface::continuous_keyboard_read();

    println!("unbuilt, insecure, not_tired");
}


// define the panic handler function for the bare_metal software
// However this &PanicInfo does not contain unwinding information. Traceback -- we have disabled unwinding 
// And anyway it wil not get called upon because panics cause immediate program termination
#[panic_handler]
fn panic_response(_info: &PanicInfo<'_> ) -> !{
    loop {     } // loop indefinately
}

