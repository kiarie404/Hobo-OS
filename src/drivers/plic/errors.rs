use core::{fmt, fmt::Display, error::Error};

#[derive(Debug, PartialEq)]
pub enum PlicError{
    Invalid_Interrupt_ID(&'static str),
    Invalid_Threshold_Value(&'static str),
    Invalid_Priority_Value(&'static str)
}

impl Display for PlicError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Plic Error : {:?}", self)
    }
}

impl Error for PlicError{
}

pub const PLIC_ERROR_Invalid_Interrupt_ID: PlicError = PlicError::Invalid_Interrupt_ID("There was an attempt to write an ID that was 0 or out of range");
pub const PLIC_ERROR_Invalid_Threshold_Value: PlicError = PlicError::Invalid_Threshold_Value("Threshold value was either less than zero or more than 7");
pub const PLIC_ERROR_Invalid_Priority_Value: PlicError = PlicError::Invalid_Priority_Value("Priority value was either less than zero or more than 7");