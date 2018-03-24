#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(slice_patterns)]
#![no_std]

// implementations of memset, memcpy... builtins assumed by llvm:
extern crate rlibc;

extern crate volatile;

mod framebuffer;
mod gpio;
mod limoncello;
mod mailbox;
mod mmio;
mod native;
mod text_screen;
mod uart;

use core::fmt::Write;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments, _file: &'static str, _line: u32, _column: u32) -> ! {
  loop {}
}

#[no_mangle]
pub extern fn kernel_main() {
  native::enable_cycle_counter();

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

  let mut screen = text_screen::TextScreen::new(fb, &text_screen::LIMONCELLO);
  screen.bg_color = 0xff0000;
  screen.fg_color = 0xffffff;
  screen.clear();
  screen.move_to(0, 33);
  let t1 = native::cycle_count();
  screen.write_string("CrapOS now booting, please stand by...\nLogistical excellence improving...\n");
  let t2 = native::cycle_count();
  write!(screen, "cycles: {:0}\n", (t2 - t1) as u32).unwrap_or(());
  write!(screen, "The meaning of life is {}\n", 42).unwrap_or(());
  write!(screen, "What if I print a line of text that's so long that it will wrap around an 80-column screen?\n").unwrap_or(());
  console.putc(0x52);
  console.putc(0x50);
  console.putc(10);

  loop {
    native::wait_for_event();
    console.putc(console.getc());
    console.putc(10);
  }
}
