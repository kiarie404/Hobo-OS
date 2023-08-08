#[derive(Debug, PartialEq)]
pub enum MappingError{
    InvalidPhysicalAddress(&'static str),
    InvalidVirtualAddress(&'static str),
    InvalidRootTableAddress(&'static str),
    InvalidAccessMap(&'static str)
}


pub const MAPPING_ERROR_InvalidAccessMap : MappingError = MappingError::InvalidAccessMap("Invalid access_map passed to mapping function");
pub const MAPPING_ERROR_InvalidRootTableAddress : MappingError = MappingError::InvalidRootTableAddress("Invalid Root table address passed to mapping function");
pub const MAPPING_ERROR_InvalidVirtualAddress : MappingError = MappingError::InvalidVirtualAddress("Invalid Virtual address passed to mapping function");
pub const MAPPING_ERROR_InvalidPhysicalAddress : MappingError = MappingError::InvalidPhysicalAddress("Invalid Physical address passed to mapping function");

#[derive(Debug, PartialEq)]
pub enum TranslationError{
    NonRangeVirtualAddress(&'static str),
    InvalidRootTableAddress(&'static str),
    UnallocatedVirtualAddress(&'static str),
    InvalidPhysicalAddress(&'static str)
}

pub const TRANS_ERROR_InvalidRootTableAddress : TranslationError = TranslationError::InvalidRootTableAddress("Invalid Root table address passed to translating function");
pub const TRANS_ERROR_NonRangeVirtualAddress : TranslationError = TranslationError::NonRangeVirtualAddress("Out of Range virtual address passed to the translating function");
pub const TRANS_ERROR_UnallocatedVirtualAddress : TranslationError = TranslationError::UnallocatedVirtualAddress("Attempted to translate an unmapped virtual address");
pub const TRANS_ERROR_InvalidPhysicalAddress : TranslationError = TranslationError::InvalidPhysicalAddress("The Physical address has no access permissions");
