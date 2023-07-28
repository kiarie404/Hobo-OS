# THis is the second version of the Bootloader.  
# The difference is that it supports running the kernel in both Machine mode and Supervisor mode
# THe Kernel is now divided ito two parts :
#   1. kinit()
#   2. kmain()

# kinit()
    #  The function of kinit does the following
    # 1. It initializes physical memory
    # 2. It defines the memory allocators (page allocator and byte allocator)
    # 3. It identity maps the pages used by the kernel under a Root_table that will be used by the kernel while in supervisor mode
    # 4. It passes the satp value to the kmain function via a return address

# kmain()
    # 1. kmain is the kernel code that runs in supervisor mode



#  The main functions of this Bootloader are : 
    # _choose_bootloading_HART
    # _clear_BSS_section
    # _initialize_registers_for_kmain (prepare its environment)
        #   for esecuting code : [stack_pointer, global_pointer]
        #   for exception handling and interrupt handling [mstatus, mepc, mie]
    # _call_kinit (give kernel code control)
    # _call kmain( run kernel code in Supervisor mode)



# notify the assembler that we will not be using Riscv Compressed instructions
# we need simplicity and predictability more than we need memory efficient code
.option norvc

# this is where we will store global initialized variables
# and we have no global data yet
.section .data

# this is code that will get called before the kmain function
# .text.init sections typically store startup code that sets up the environment for the rest of the code
.section .text.init 

# _start is declared as a global symbol so that the linker gets to detect it 
# This will be the entry point of the bootloader
.global _start
_start:
    j   _choose_bootloading_HART
    
# The gp register currently contains the gp_memory address of the loader.
# We need to update it to point to the kernel's gp
# We numb out optimizations to make sure the update happens explicitly
_fetch_kernel_global_pointer:
    .option push    # save and disable all current assembler directives
    .option norelax # disable code optimization, this is a delicate operation; we need no surprises
    la gp, _global_pointer # load the address of _global_pointer label into the gp register
    .option pop  # restore previous assembler directives
    j _clear_BSS_section


_choose_bootloading_HART:
    # fetch the ID of the current Hardware Thread (HART) and store it in the temporary register t1
    csrr t1, mhartid 
    bnez t1, _make_HART_sleep # If HART ID is not ZERO, make that HART sleep.
                             # If HART IS is zero, _fetch_kernel_global_pointer
    j   _fetch_kernel_global_pointer  # after choosing the HART, we move on to configure essential registers 
                                      # [gp, sp, ]
    

# this does not completely shut down the HART
_make_HART_sleep:
    wfi                 # power off and wait for an interrupt
    j _make_HART_sleep  # continuously make HART sleep, we are running a single_core OS

# the bootloader needs to make sure that all uninitialized dlobal values of...
# ...the kernel are ZEROED out
_clear_BSS_section:
    la a1, _bss_start
    la a2, _bss_end
    j _clear_BSS_section_loop

_clear_BSS_section_loop:
    sd      zero, (a1)                          # store z mepc, mieero in the 64bit memory space referenced by a1
    addi    a1, a1, 8                           # increment the address by 64 bits. (8 bytes)
    bltu    a1, a2, _clear_BSS_section_loop     # loop until we reach the last address of the bss section
    j       _initialize_registers_for_kinit     # if we have zeroed out the BSS section, _initialize_registers_for_kinit()

_initialize_registers_for_kinit:
    la		sp, _stack_end                          # setup the stack pointer
    li		t0, (0b11 << 11) | (0 << 7) | (0 << 3)  # Set MPP field to 11 (Machine Mode), 
                                                    # Bit 7, sets MPIE bit to 0 ; meaning interrupts from lower levels can get handled by machine mode if invoked
                                                    # Bit 3, Sets the MIE bit to 1 ; meaning the CPU can receive interrups while in machine mode
    csrw	mstatus, t0

    # set kmain to be the value that will be pasted tp the PC counter after calling mret
    la		t1, kinit
	csrw	mepc, t1   

    #set the Machine trap vector
    la		t2, asm_trap_vector
	csrw	mtvec, t2  

    # allow specific interrupts
    # 3 == Software Interrupts, 7 == Timer Interrupts, 11 == External Interrupts
    # li		t3, (1 << 1) | (1 << 3) | (1 << 5) | (1 << 7) | (1 << 9) | (1 << 11) 
	# csrw	mie, t3

    #  set the return address to point to the address of the  "_initialize_registers_for_kmain" code
    la      ra, _initialize_environment_for_kmain

    # call kmain (indirectly, this is because mret will make the cpu program counter to point to the value in mepc(kmain))
    mret

_initialize_environment_for_kmain:
    # this code is meant to prepare the CPU to execute the kernel in supervisor mode
    # registers like the global pointer, stackpointer can remain the way they are

    # set the sstatus register
    # 1 << 8    : Supervisor's previous protection mode is 1 (SPP=1 [Supervisor]).
	# 1 << 5    : Supervisor's previous interrupt-enable bit is 1 (SPIE=1 [Enabled]).
	# 1 << 1    : Supervisor's interrupt-enable bit [Enabled]
    li      t0, (1 << 8)|(1 << 5)|(1 << 1)
    csrw    sstatus, t0 


    li        t1, (1 << 1)| (1 << 5)| (1 << 9)
    # csrw      sie, t1 
    csrw      mideleg, t1 

    # Enable all exceptions by setting all of th 16 bits of medeleg to 1
    # Now all exceptions that happen in bot Supervisor and Usermode will be handled in Superisor mode
    li      t2, (1 << 1)| (1 << 5) | (1 << 9)
    # csrw    medeleg, t2 

    # Set the stvec register
    # in our case, the stvec will still point to the address of the vector that was called  while we were in machine mode
    # It's the same thing. The code in it was not Machine-mode-specific
    la      t0, asm_trap_vector
    csrw    stvec, t0  

    # Define the start point of kernel 
    la      t0, kmain
    csrw    sepc, t0
    
    # Update the value in the satp. The satp value was returned by kinit() via register a0
    sfence.vma 
    csrw    satp, a0 
    sfence.vma 
    # force the CPU to use a fresh satp and corresponding translation tables. This instruction tells the CPU :
    # "Do not use cached information, use new and up-to-date information. Refresh things"
    # What sfence.vma does in the backgroud is to fulfill all "write" operations that are pending. Our ain is to make sure
    # that the above instruction "csrw satp, a0" gets completed before calling sret

    sret
    







    


