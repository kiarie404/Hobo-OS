# trap.S
# In the future our trap vector will go here.

.global asm_trap_vector
# This will be our trap vector when we start
# handling interrupts.
asm_trap_vector:
# The mret instruction in RISC-V assembly is used to return control from a trap handler to a previous machine mode execution state
	mret