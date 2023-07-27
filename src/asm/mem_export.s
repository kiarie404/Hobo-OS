# This module exports the memory labels that were used in the assembly script.  
# Exporting those memories is necessary because it makes the memory labels to be accessible from Rust Code.  
# This module exports them by mking the labels to be GLOBAL. That way references made to them by Rust Code will be solved during the object file linking process
# However Rust Code needs to declare that is is using imported variables using the "extern C" block

// we store this memory labels as global read only data
.section .rodata

# All the variables that start with an underscore have been exported from the linker script 

.global TEXT_START
TEXT_START: .dword _text_start

.global TEXT_END
TEXT_END: .dword _text_end

.global RODATA_START
RODATA_START: .dword _rodata_start

.global RODATA_END
RODATA_END: .dword _rodata_end

.global DATA_START
DATA_START: .dword _data_start

.global DATA_END
DATA_END: .dword _data_end

.global BSS_START
BSS_START: .dword _bss_start

.global BSS_END
BSS_END: .dword _bss_end

.global KERNEL_STACK_START
KERNEL_STACK_START: .dword _stack_start 

.global KERNEL_STACK_END
KERNEL_STACK_END: .dword _stack_end 

.global HEAP_START 
HEAP_START: .dword _heap_start # exported from the linker script

.global HEAP_END
HEAP_END: .dword _heap_end   # exported from the linker script



