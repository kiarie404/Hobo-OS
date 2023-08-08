
# trap.S
# This module contains the asm_trap_vector.
# The asm trap vector is a function that gets called by the CPU when an interrupt or exception occurs
# So the asm trap vector is stored in the mtvec register of the cpu.	


.option norvc
.altmacro
.set NUM_GP_REGS, 32  # Number of registers per context
.set NUM_FP_REGS, 32
.set REG_SIZE, 8   # Register size (in bytes)
.set MAX_CPUS, 8   # Maximum number of CPUs

# Use macros for saving and restoring multiple registers
.macro save_gp i, basereg=t6
	sd	x\i, ((\i)*REG_SIZE)(\basereg)
.endm
.macro load_gp i, basereg=t6
	ld	x\i, ((\i)*REG_SIZE)(\basereg)
.endm
.macro save_fp i, basereg=t6
	fsd	f\i, ((NUM_GP_REGS+(\i))*REG_SIZE)(\basereg)
.endm
.macro load_fp i, basereg=t6
	fld	f\i, ((NUM_GP_REGS+(\i))*REG_SIZE)(\basereg)
.endm



.section .text
.global asm_trap_vector
# This must be aligned by 4 since the last two bits
# of the mtvec register do not contribute to the address
# of this vector.
.align 4  # the mtvec register requires that the address stored it its BASE to be aligned to a multiple of 4. ie Last 2 zeroes get truncated by MUST
asm_trap_vector:
	j save_context_to_trap_frame

# The function below is purposely written in assembly to avoid any small unexpected changes among the CPU registers.
# If we wrote it in Rust, some register values would change according to the assembly code that the compiler would generate.  
# That is not predictable at ALL.

# The function saves all regular register values to the trapframe whose address gets been stored in the mstatus register
# We use macros to do this
save_context_to_trap_frame:

	csrrw	t6, mscratch, t6
	.set 	i, 1
	.rept	30
		save_gp	%i
		.set	i, i+1
	.endr

	# Save the actual t6 register, which we swapped into
	# mscratch
	mv		t5, t6
	csrr	t6, mscratch
	save_gp 31, t5

	# Restore the kernel trap frame into mscratch
	csrw	mscratch, t5

	# store the control status registers to the trapframe
	csrr	t0, mscratch   # use t0 as the base register
	csrr	t1, satp 
	sd		t1, 512(t0)
	csrr 	t1, mstatus
	sd		t1, 520(t0)
	csrr 	t1, mepc
	sd		t1, 528(t0)
	csrr 	t1, mie
	sd		t1, 536(t0)
	csrr 	t1, mcause
	sd		t1, 544(t0)
	csrr 	t1, mtval
	sd		t1, 552(t0)

	# rust_trap_handler returns the address of the next instruction
	call	ra, rust_trap_handler

	# update the mepc to be equal to th returned value
	csrw	mepc, a0 

	# restore the non-priviledged registers
	# Now load the trap frame back into t6
	csrr	t6, mscratch

	# Restore all GP registers
	.set	i, 1
	.rept	31
		load_gp %i
		.set	i, i+1
	.endr

	mret




# .global make_syscall
# make_syscall:
# 	ecall
# 	ret



# msxratch = 2147496632
