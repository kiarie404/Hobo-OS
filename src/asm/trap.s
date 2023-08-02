
# trap.S
# This module contains the asm_trap_vector.
# The asm trap vector is a function that gets called by the CPU when an interrupt or exception occurs
# So the asm trap vector is stored in the mtvec register of the cpu.	
.option norvc

.section .text
.global asm_trap_vector
# This must be aligned by 4 since the last two bits
# of the mtvec register do not contribute to the address
# of this vector.
.align 4
asm_trap_vector:
	# rust_trap does the following functions :
		# 1. It saves the context of the CPU and stores it in the appropriate trap frame
		# 2. It handles the exception or interrupt
		# 3. It returns the control back to the process that was interrupted OR a different process if needed
	call	ra, rust_trap

	# When we get here, we've returned from m_trap, restore registers
	# and return.
	# m_trap will return the return address via a0.

	# csrw	mepc, a0

	# # Now load the trap frame back into t6
	# csrr	t6, mscratch

	# # Restore all GP registers
	# .set	i, 1
	# .rept	31
	# 	load_gp %i
	# 	.set	i, i+1
	# .endr

	# # Since we ran this loop 31 times starting with i = 1,
	# # the last one loaded t6 back to its original value.

	# mret
	# j _make_HART_sleep

# .global make_syscall
# make_syscall:
# 	ecall
# 	ret
