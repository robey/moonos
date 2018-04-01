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
pub extern fn syscall_zero(n: usize) {
  print!("\nSYSCALL ZERO {}\n", n);
}

#[no_mangle]
pub extern fn syscall_one(n: usize) {
  print!("\nSYSCALL ONE {}\n", n);
}
