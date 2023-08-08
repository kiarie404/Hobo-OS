#[derive(Debug)]
pub enum MemoryAllocatioErrors{
    ZeroPagesRequested(&'static str), // The page allocator was requested to allocate zero pages
    NoFreeContiguousSpace(&'static str) // No sufficient free contiguous space was found
}

#[derive(Debug)]
pub enum MemoryDeallocatioError{
    NonHeapAddressFound(&'static str), // The address is not within heap range
    PageAddressIsMiddlePage(&'static str), // The address is not divisible by 4096
    PageNotLeading(&'static str), // The Page address refers to a Page that is not the leading page in the contiguous group of pages
    Other(&'static str)
}
