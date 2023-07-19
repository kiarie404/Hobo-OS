#[derive(Debug)]
pub enum UartInterrupt{
    DataReady,        // the Buffer is full and it can be read from
    BufferEmpty,      // The buffer is empty and it can be written to
    ReceiverLineStatus, // there was a transmission error [framing error, paruty-bit signalled data loss ....]
    ArbitraryInterrupt  // errors like Modem errors. Error we currently dont care about. This field is here for completeness
}