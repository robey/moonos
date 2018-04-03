use debug;
use native;

#[no_mangle]
pub extern fn vector_reset() {
  print!("\n*** RESET\n");
  native::halt();
}

#[no_mangle]
pub extern fn vector_undefined() {
  print!("\n*** UNDEFINED INSTRUCTION\n");
  native::halt();
}

#[no_mangle]
pub extern fn syscall_zero(n: usize, _a: usize, _b: usize, _c: usize) -> usize {
  print!("\nSYSCALL ZERO {}\n", n);
  debug::dump(_c);
  n * 2
}

#[no_mangle]
pub extern fn syscall_one(n: usize) -> usize {
  print!("\nSYSCALL ONE {}\n", n);
  return n * 2;
}
