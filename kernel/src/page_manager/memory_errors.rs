#[derive(Debug)]
pub enum MemoryAllocationError{
    ZeroPagesRequested(&'static str), // The page allocator was requested to allocate zero pages
    NoFreeContiguousSpace(&'static str) // No sufficient free contiguous space was found, you might have to do some defragmentation to find some space[undone]
}

#[derive(Debug)]
pub enum MemoryDeallocationError{
    NonHeapAddressFound(&'static str), // The address is not within heap range
    NonPageAddress(&'static str), // The address is not divisible by 4096, You are trying to deallocate a page that is in the middlr/end of a contiguous allocation
    PageNotLeading(&'static str), // The Page address refers to a Page that is not the leading page in the contiguous group of pages
    Other(&'static str)
}


pub const NON_PAGE_ADDRESS:MemoryDeallocationError = MemoryDeallocationError::NonPageAddress("The address is not not a page address, it is an address of a byte within a page");
pub const NON_HEAP_ADDRESS:MemoryDeallocationError = MemoryDeallocationError::NonHeapAddressFound("Page address is not within the Heap Memory range");
pub const PAGE_NOT_LEADING:MemoryDeallocationError = MemoryDeallocationError::PageNotLeading("The Page address references to a Page that is not the leading page in the contiguous group of pages");
// [undone] : heap size for each process to be pre-defined
// [undone] : defragmentation support
// [undone] : non-contiguous support