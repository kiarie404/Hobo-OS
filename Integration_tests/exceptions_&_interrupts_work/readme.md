THis module tests stdin and stdout modules of the kernel.   
Both modules interact with the UART and PLIC drivers.   
They fail to do things in an interrupt driven fashion  
They pass when things are done in a poll driven fashion.    

[undone] : Make I/O to be interrupt driven