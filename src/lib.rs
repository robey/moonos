#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(slice_patterns)]
#![no_std]

#[macro_use]
mod text_display;

// compiler intrinsics like __aeabi_memcpy and __aeabi_uidiv
extern crate compiler_builtins;

// implementations of memset, memcpy... builtins assumed by rust:
// (this feels like a bug. they should be using __aeabi_memset)
extern crate rlibc;

// spinlocks
extern crate spin;

// extern crate volatile;

mod gpio;
mod limoncello;
mod mailbox;
mod mmio;
mod native;
mod screen;
mod uart;

use screen::screen_init;
use text_display::TEXT_DISPLAY;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments, _file: &'static str, _line: u32, _column: u32) -> ! {
  loop {}
}

#[no_mangle]
pub extern fn kernel_main() {
  native::enable_cycle_counter();

  // FIXME should be global mutable mutex
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

  screen_init(640, 480, 24).unwrap();

  {
    let mut t = TEXT_DISPLAY.lock();
    t.bg_color = 0xff0000;
    t.fg_color = 0xffffff;
    t.clear();
    t.move_to(0, 33);
  }

  let t1 = native::cycle_count();
  print!("CrapOS now booting, please stand by...\n");
  let t2 = native::cycle_count();

  let mem = mailbox::get_memory_info().unwrap();
  print!("Memory: RAM {}MB, GPU {}MB\n", mem.cpu_size >> 20, mem.gpu_size >> 20);
  print!("© 2018 Gnashers of Insomnia\nFranzösische Straße 1403, Berlin\n");
  print!("cycles: {:0}\n", t2 - t1);
  print!("The meaning of life is {}\n", 42);
  print!("What if I print a line of text that's so long that it will wrap around an 80-column screen?\n");

  console.putc(0x52);
  console.putc(0x50);
  console.putc(10);

  loop {
    native::wait_for_event();
    console.putc(console.getc());
    console.putc(10);
  }
}
