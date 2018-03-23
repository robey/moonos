#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(slice_patterns)]
#![no_std]

mod limoncello;
mod font;
mod framebuffer;
mod gpio;
mod mailbox;
mod mmio;
mod uart;

// #[lang = "eh_personality"]
// #[no_mangle]
// pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments, _file: &'static str, _line: u32, _column: u32) -> ! {
  loop {}
}

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

  mailbox::get_memory_info().map(|info| {
    console.put_u32(info.cpu_base);
    console.putc(32);
    console.put_u32(info.cpu_size);
    console.putc(32);
    console.put_u32(info.gpu_base);
    console.putc(32);
    console.put_u32(info.gpu_size);
    console.putc(10);
  });

  let mut fb = framebuffer::framebuffer();
  fb.set_size(640, 480, 24);
  fb.get_framebuffer();
  fb.fill_box(0, 0, 640, 480, 0xff0000);

  let myfont = font::Font::new(limoncello::FONT_WIDTH, limoncello::FONT_HEIGHT, &limoncello::FONT_DATA);
  let mut x = 16;
  let y = 14;
  for c in "Hello raspi kernel world!".bytes() {
    myfont.putc(&mut fb, x, y, c as u8, 0x00ffff, 0xff0000);
    x += 8;
  }

  console.putc(0x52);
  console.putc(0x50);
  console.putc(10);

  loop {
    console.putc(console.getc());
    console.putc(10);
  }
}
