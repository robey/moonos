
## hardware (raspi 3)

- qualcomm BCM2837
- 1.2GHz 64-bit, 4-core, cortex A53
- 1GB ram
- peripherals are mapped into physmem at 0x3f00_0000


## rust

- cargo needs `panic = "abort"` so that it doesn't try to use c++ exception logic to handle a panic. that requires the symbol `__aeabi_unwind_cpp_pr0`, which would require `libgcc`, an implementation of (at least) `abort` and `memcpy`, and extra linker instructions for where to put the exception table.
