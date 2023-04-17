# boot.S

# norvc implies that the assemly code should not use reduced(compressed) instructions
.option norvc

# uninitialized variables get declared under .section .data
.section .data

# the instructions get declared under .section .text.init
.section .text.init

# I have made it global so that the linker becomes aware of the _start function
# It is assembly so there is no need of #no_mangle
.global _start
_start:
	# Any hardware threads (hart) that are not bootstrapping
	# need to wait for an interrupt. This interrupt will only be supplied by the kernel that is yet to be loaded after booting
	csrr	t0, mhartid	# read the Hart ID from the mhartid and store it in register t0
	bnez	t0, 3f		# bnez checks if the value contained in the register is zero, if value == zero, continue. Else, skip to code block 3f

	# Supervisor Address Translation and Protection (SATP) register should be zero, but let's make sure --- we start on a clean sheet.
	csrw	satp, zero

.option push


# Disable linker instruction relaxation for the `la` instruction below.
# This disallows the assembler from assuming that `gp` is already initialized.
# This causes the value stored in `gp` to be calculated from `pc`.
.option norelax
	la		gp, _global_pointer
.option pop
	# The BSS section is expected to be zero
	la 		a0, _bss_start
	la		a1, _bss_end
	bgeu	a0, a1, 2f
1:
	sd		zero, (a0)
	addi	a0, a0, 8
	bltu	a0, a1, 1b
2:
	# Control registers, set the stack, mstatus, mepc,
	# and mtvec to return to the main function.
	# li		t5, 0xffff;
	# csrw	medeleg, t5
	# csrw	mideleg, t5
	la		sp, _stack
	# We use mret here so that the mstatus register
	# is properly updated.
	li		t0, (0b11 << 11) | (1 << 7) | (1 << 3)
	csrw	mstatus, t0
	la		t1, kmain
	csrw	mepc, t1
	la		t2, asm_trap_vector
	csrw	mtvec, t2
	li		t3, (1 << 3) | (1 << 7) | (1 << 11)
	csrw	mie, t3
	la		ra, 4f
	mret
3:

	# Parked harts go here. We need to set these
	# to only awaken if it receives a software interrupt,
	# which we're going to call the SIPI (Software Intra-Processor Interrupt).
	# We only use these to run user-space programs, although this may
	# change.
4:
	wfi
	j		4b

