#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(slice_patterns)]
#![no_std]

mod framebuffer;
mod mailbox;
mod mmio;
mod uart;

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! { loop {} }

// FIXME why
#[no_mangle]
pub extern fn memset(s: *mut u8, c: usize, n: isize) {
  unsafe { for i in 0..n { *s.offset(i) = c as u8 } }
}

#[no_mangle]
pub extern fn kernel_main() {
  let console = uart::Uart::new(uart::RPI2_UART0);
  console.init();
  console.puts("hello raspi kernel world!\r\n");

  let mut fb = framebuffer::framebuffer();
  fb.set_size(640, 480, 24);
  console.put_u32(fb.get_framebuffer() as u32);
  console.putc(10);
  for x in 5..20 {
    for y in 5..20 {
      fb.set_pixel(x, y, 0x00ff00);
    }
  }
  console.putc(0x52);
  console.putc(0x50);
  console.putc(10);

  loop {
    console.putc(console.getc());
    console.putc(10);
  }
}
