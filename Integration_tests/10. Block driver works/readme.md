The Block Driver works well.  
However, you may get inconsistent data reads if you do not use ptr.read_volatile and ptr.write_volatile when reading or writing data to the buffer that gets shared between the Block device and kernel