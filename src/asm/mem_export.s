# This module exports the memory labels that were used in the assembly script.  
# Exporting those memories is necessary because it makes the memory labels to be accessible from Rust Code.  
# This module exports them by mking the labels to be GLOBAL. That way references made to them by Rust Code will be solved during the object file linking process
# However Rust Code needs to declare that is is using imported variables using the "extern C" block

// we store this memory labels as global read only data
.section .rodata
.global HEAP_START 
HEAP_START: .dword _heap_start # exported from the linker script

.global HEAP_END
HEAP_END: .dword _heap_end   # exported from the linker script