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

  let info = mailbox::get_memory_info().unwrap();
  console.put_u32(info.cpu_base);
  console.putc(32);
  console.put_u32(info.cpu_size);
  console.putc(32);
  console.put_u32(info.gpu_base);
  console.putc(32);
  console.put_u32(info.gpu_size);
  console.putc(10);

  let mut fb = framebuffer::framebuffer();
  fb.set_size(640, 480, 24);
  fb.get_framebuffer();
  fb.blit_glyph(6, 8, 8, &[ 0x7c, 0x12, 0x11, 0x12, 0x7c, 0 ], 0x00ffff, 0xff0000);
  fb.blit_glyph(12, 8, 8, &[ 0x7c, 0x12, 0x11, 0x12, 0x7c, 0 ], 0x00ffff, 0xff0000);
  fb.blit_glyph(18, 8, 8, &[ 0x7c, 0x12, 0x11, 0x12, 0x7c, 0 ], 0x00ffff, 0xff0000);
  fb.blit_glyph(24, 8, 8, &[ 0x7c, 0x12, 0x11, 0x12, 0x7c, 0 ], 0x00ffff, 0xff0000);
  console.putc(0x52);
  console.putc(0x50);
  console.putc(10);

  loop {
    console.putc(console.getc());
    console.putc(10);
  }
}
