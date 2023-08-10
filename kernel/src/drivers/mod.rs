pub mod uart;
pub mod timer;
pub mod plic;
pub mod virtio_block;

use virtio_block::virtio_protocol_abstractions::{*};
use crate::{print, println};

// export SOLID static references to Driver Instances
// pub static mut UART_DEVICE : UartDevice = UartDevice::init();

/// Creates an instance of all devvices and configures them to their defaults
/// It calls the init finctions of each driver
pub fn init_all_drivers(){
    uart::UartDevice::init();
    plic::init();
}

/// Probe the VirtIO bus for devices that might be
/// out there.  
/// This function has been imported as part of the Virtio block by Stephen Marz
pub fn probe() {
    
	// Rust's for loop uses an Iterator object, which now has a step_by
	// modifier to change how much it steps. Also recall that ..= means up
	// to AND including MMIO_VIRTIO_END.
	for addr in (MMIO_VIRTIO_START..=MMIO_VIRTIO_END).step_by(MMIO_VIRTIO_STRIDE) {
		print!("Virtio probing 0x{:08x}...", addr);
		let magicvalue;
		let deviceid;
		let ptr = addr as *mut u32;
		unsafe {
			magicvalue = ptr.read_volatile();
			deviceid = ptr.add(2).read_volatile();
		}
		// 0x74_72_69_76 is "virt" in little endian, so in reality
		// it is triv. All VirtIO devices have this attached to the
		// MagicValue register (offset 0x000)
		if MMIO_VIRTIO_MAGIC != magicvalue {
			println!("not virtio.");
		}
		// If we are a virtio device, we now need to see if anything
		// is actually attached to it. The DeviceID register will
		// contain what type of device this is. If this value is 0,
		// then it is not connected.
		else if 0 == deviceid {
			println!("not connected.");
		}
		// If we get here, we have a connected virtio device. Now we have
		// to figure out what kind it is so we can do device-specific setup.
		else {
			match deviceid {
				// DeviceID 1 is a network device
				1 => {
					print!("network device...");
					if false == setup_network_device(ptr) {
						println!("setup failed.");
					}
					else {
						println!("setup succeeded!");
					}
				},
				// DeviceID 2 is a block device
				2 => {
					print!("block device...");
					if false == virtio_block::setup_block_device(ptr) {
						println!("setup failed.");
					}
					else {
						let idx = (addr - MMIO_VIRTIO_START) >> 12;
						unsafe {
							VIRTIO_DEVICES[idx] =
								Some(VirtioDevice::new_with(DeviceTypes::Block));
						}
						println!("setup succeeded!");
					}
				},
				// DeviceID 4 is a random number generator device
				4 => {
					print!("entropy device...");
					if false == setup_entropy_device(ptr) {
						println!("setup failed.");
					}
					else {
						println!("setup succeeded!");
					}
				},
				// DeviceID 16 is a GPU device
				16 => {
					print!("GPU device...");
					if false == setup_gpu_device(ptr) {
						println!("setup failed.");
					}
					else {
						println!("setup succeeded!");
					}
				},
				// DeviceID 18 is an input device
				18 => {
					print!("input device...");
					if false == setup_input_device(ptr) {
						println!("setup failed.");
					}
					else {
						println!("setup succeeded!");
					}
				},
				_ => println!("unknown device type."),
			}
		}
	}
}

pub fn setup_network_device(_ptr: *mut u32) -> bool {
	false
}

pub fn setup_gpu_device(_ptr: *mut u32) -> bool {
	false
}

pub fn setup_input_device(_ptr: *mut u32) -> bool {
	false
}

pub fn setup_entropy_device(_ptr: *mut u32) -> bool {
	false
}

// The External pin (PLIC) trap will lead us here if it is
// determined that interrupts 1..=8 are what caused the interrupt.
// In here, we try to figure out where to direct the interrupt
// and then handle it.
pub fn handle_interrupt(interrupt: u32) {
	let idx = interrupt as usize - 1;
	unsafe {
		if let Some(vd) = &VIRTIO_DEVICES[idx] {
			match vd.devtype {
				DeviceTypes::Block => {
					virtio_block::handle_interrupt(idx);
				},
				_ => {
					println!("Invalid device generated interrupt!");
				},
			}
		}
		else {
			println!("Spurious interrupt {}", interrupt);
		}
	}
}
