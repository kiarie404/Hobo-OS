//! This module bundles up all the assembly files

// instead of linking the assembly files as seperate files,
// It was opted to just stringify the assembly files and let the rust compiler consume them as inline assembly
use core::arch::global_asm;

// global_asm!(include_str!("boot_copy.s"));
// include_str! macro takes in a file and outputs a &'static str
global_asm!(include_str!("booze.s"));
// global_asm!(include_str!("boot.s"));
global_asm!(include_str!("trap.s"));
global_asm!(include_str!("mem_export.s"));