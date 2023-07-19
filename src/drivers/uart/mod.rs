mod uart_errors;
mod uart_interrupts;

use volatile_register::{RW, RO};
use uart_errors::UartError;
use uart_interrupts::UartInterrupt;

// attach dependent modules


// import usages
use core::fmt::Write; // we need this Trait in order to make the UART device to act as a Unicode Stream Buffer
use core::fmt::Error; // The Write trait above works with formatted data. Writing its functions requires us to use fmt::Error


// offsets of various UART registers in relation to the base address
const BASE_ADDRESS_USZ : usize = 0x1000_0000;
const BUFFER_OFFSET : usize = 0;
const IER_OFFSET : usize = 1; // Interrupt Enable Register offset
const ISR_OFFSET : usize = 2; // Interrupt Status Register offset
const FCR_OFFSET : usize = 2; // FIFO Control Register offset
const LCR_OFFSET : usize = 3; // Line Control Register offset
const LSR_OFFSET : usize = 5; // Line Status Register offse



// Virtualize the UART component connected to the I/O as a public struct
#[repr(C)]
struct Uart_MMIO{
    buffer_reg : RW<u8>,
	interrupt_enable_reg : RW<u8>,
	intrpt_fcr_reg : RW<u8>,
	line_ctrl_reg : RW<u8>,
	modem_ctrl_reg : RW<u8>,
	line_status_reg : RW<u8>,
	modem_status_reg : RW<u8>,
	scratch_reg : RW<u8>,
}

pub struct UartDevice{
	mmio : &'static mut Uart_MMIO
}

impl UartDevice{
	/// creates a new struct UartDevice that references the static mmio for the uart device	 
	/// the uart is at 0x1000_0000
	pub fn new () -> UartDevice{
		let ptr_to_mmio = 0x1000_0000 as *mut Uart_MMIO;
		let ref_to_mmio = unsafe {&mut *(ptr_to_mmio)};
		let uart_instance = UartDevice { mmio: ref_to_mmio };
		return uart_instance;
	}

	// sets the data width to be 8 bits.	
	// It does this by setting each of the last two bits of the Line Control Register to 1
	fn set_data_width(&mut self){
		unsafe {self.mmio.line_ctrl_reg.write(0b0000_0011)};
	}

	// this functions enables the "Data Ready interrupt"
	fn enable_interrupts(&mut self){
		unsafe {self.mmio.interrupt_enable_reg.write(0b0000_0001)};
	}

	// this function enables the buffer to be FIFO compliant
	fn enable_fifo(&mut self){
		unsafe {self.mmio.intrpt_fcr_reg.write(0b0000_0001)};
	}

	// check if data is ready to be read
	fn check_if_read_ready(&self) -> bool{
		let line_status_value = self.mmio.line_status_reg.read();
		// extracting the last bit from the line status register
		let result = line_status_value & 0b0000_0001;
		if result == 0b0000_0001 { return true;	}
		else {	return false }
	}

	// check if the data buffer is empty
	fn check_if_write_ready (&self) -> bool{
		unimplemented!()
	}

	// this function writes the given byte value to the buffer
	pub fn write_to_buffer (&mut self, val: u8){
		unsafe {self.mmio.buffer_reg.write(val)};
	}

	// This function reads from the buffer and returns an Option<u8>
	pub fn read_from_buffer(&self) -> Option<u8>{
		let read_ready_status = self.check_if_read_ready();
		if read_ready_status == true {
			let value = self.mmio.buffer_reg.read();
			return Some(value);
		}
		else {	return None;	}
	}



}

// implement the write_str for UartDevice
impl Write for UartDevice {
	fn write_str(&mut self, out: &str) -> Result<(), Error> {
		for character in out.bytes() {
			unsafe {self.mmio.buffer_reg.write(character)};
		}
		Ok(())
	}
}

