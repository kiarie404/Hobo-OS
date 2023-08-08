pub mod uart;
pub mod timer;
pub mod plic;

// export SOLID static references to Driver Instances
// pub static mut UART_DEVICE : UartDevice = UartDevice::init();

/// Creates an instance of all devvices and configures them to their defaults
/// It calls the init finctions of each driver
pub fn init_all_drivers(){
    uart::UartDevice::init();
    plic::init();
}
