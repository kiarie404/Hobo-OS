
#[derive(Debug)]
pub enum UartError{
    UnableToWriteToBuffer,  // unable to write to buffer because the UART was not Write_ready
    UnableToReadBuffer,     // unable to read from buffer because the UART was not Read_ready
    Others
}