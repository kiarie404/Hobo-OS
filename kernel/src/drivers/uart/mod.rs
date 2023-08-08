mod uart_errors;
mod uart_interrupts;

use volatile_register::{RW, RO};
use uart_errors::UartError;
use uart_interrupts::UartInterrupt;

// attach dependent modules


use core::{fmt, fmt::Debug, fmt::Display};
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

impl Debug for Uart_MMIO{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Uart_MMIO")
         .field("buffer_reg", &self.buffer_reg.read())
         .field("interrupt_enable_reg", &self.interrupt_enable_reg.read())
		 .field("intrpt_fcr_reg", &self.intrpt_fcr_reg.read())
		 .field("line_ctrl_reg", &self.line_ctrl_reg.read())
		 .field("modem_ctrl_reg", &self.modem_ctrl_reg.read())
		 .field("line_status_reg", &self.line_status_reg.read())
		 .field("modem_status_reg", &self.modem_status_reg.read())
		 .field("scratch_reg", &self.scratch_reg.read()) 
         .finish()
    }
}

#[derive(Debug)]
pub struct UartDevice{
	mmio : &'static mut Uart_MMIO
}

impl Display for UartDevice{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\t buffer : {:08b} \n", self.mmio.buffer_reg.read()).unwrap();
		write!(f, "\t interrupt_enable_reg : {:08b} \n", self.mmio.interrupt_enable_reg.read()).unwrap();
		write!(f, "\t intrpt_status_reg : {:08b} \n", self.mmio.intrpt_fcr_reg.read()).unwrap();
		write!(f, "\t line_ctrl_reg : {:08b} \n", self.mmio.line_ctrl_reg.read()).unwrap();
		write!(f, "\t modem_ctrl_reg : {:08b} \n", self.mmio.modem_ctrl_reg.read()).unwrap();
		write!(f, "\t line_status_reg : {:08b} \n", self.mmio.line_status_reg.read()).unwrap();
		write!(f, "\t modem_status_reg : {:08b} \n", self.mmio.modem_status_reg.read()).unwrap();
		write!(f, "\t scratch_reg : {:08b}", self.mmio.scratch_reg.read())
    }
}

impl UartDevice{
	/// creates a new struct UartDevice that references the static mmio for the uart device	 
	/// the uart is at 0x1000_0000
	/// This function does not create a new individual instance, it just creates a new reference in the background
	pub fn new () -> UartDevice{
		let ptr_to_mmio = 0x1000_0000 as *mut Uart_MMIO;
		let ref_to_mmio = unsafe {&mut *(ptr_to_mmio)};
		let uart_instance = UartDevice { mmio: ref_to_mmio };
		return uart_instance;
	}

	/// Returns a UartDevice that has been configured
	pub fn init() -> UartDevice{
		let mut instance = UartDevice::new();
		instance.enable_fifo();
		instance.enable_interrupts();
		instance.set_data_width();

		return instance;
	}

	/// sets the data width of the transmission frames to be 8 bits.	
	/// It does this by setting each of the last two bits of the Line Control Register to 1
	pub fn set_data_width(&mut self){
		unsafe {self.mmio.line_ctrl_reg.write(0b0000_0011)};
	}

	/// this functions enables the "Data Ready interrupt" in the Interrupt Enable register
	pub fn enable_interrupts(&mut self){
		unsafe {self.mmio.interrupt_enable_reg.write(0b0000_0001)};
	}

	/// this function enables the UART buffer to be FIFO compliant.  
	/// THis affects two buffers : The Receiver Buffer and the Transmission buffer
	pub fn enable_fifo(&mut self){
		unsafe {self.mmio.intrpt_fcr_reg.write(0b0000_0001)};
	}

	/// check if data is ready to be read from the UART. 
	pub fn check_if_read_ready(&self) -> bool{
		let line_status_value = self.mmio.line_status_reg.read();
		// extracting the last bit from the line status register
		let result = line_status_value & 0b0000_0001;
		if result == 0b0000_0001 { return true;	}
		else {	return false }
	}

	/// check if the UART is ready to be written to
	pub fn check_if_write_ready (&self) -> bool{
		unimplemented!()
	}

	/// this function writes the given byte value to the buffer
	pub fn write_to_buffer (&mut self, val: u8){
		unsafe {self.mmio.buffer_reg.write(val)};
	}

	/// This function reads from the buffer and returns an Option<u8> 
	/// If there is no data in the buffer, it returns None value
	pub fn read_from_buffer(&self) -> Option<u8>{
		let read_ready_status = self.check_if_read_ready();
		if read_ready_status == true {
			let value = self.mmio.buffer_reg.read();
			return Some(value);
		}
		else {	return None;	}
	}

	pub fn read_interrupt_status_reg(&self) -> u8{
		self.mmio.intrpt_fcr_reg.read()
	}

	/// Empties the read_Bufer. 
	/// This does not write a 1 in the "rx FIFO reset" bit of the control status register. Doing so would be too drastic, It would reset FIFO pointers.
	/// Instead, we just make th CPU read all data until the receive_buffer is empty
	pub fn empty_FIFO_receive_buffer(&self){
		let has_data = self.check_if_read_ready();
		if has_data == true {
			// continuously read data from the buffer till its empty
			loop {
				let data_option = self.read_from_buffer();
				match data_option {
					Some(byte) => { /*do nothing with the data, we are reading it for the sake of emptying the UART buffer */},
					None => { // this means tthat he buffer is finally empty, break free from the endless loop
						break;
					}
				}
			}
		}
	}



}

// implement the write_str for UartDevice
// Implementing the Write trait is essential, It makes the UART struct to be treated as a Unicode Stream Buffer
// However, this buffer is not flushable. (I haven't really understood this)
impl Write for UartDevice {
	fn write_str(&mut self, out: &str) -> Result<(), Error> {
		for character in out.bytes() {
			unsafe {self.mmio.buffer_reg.write(character)};
		}
		Ok(())
	}
}

