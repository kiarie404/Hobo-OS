.section .text
.global asm_trap_vector

asm_trap_vector :
    # I will redirect this to a function defined in Rust.   
    # Assembly code aint that smooth
    # for now we will do nothing
    mret
