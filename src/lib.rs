#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(slice_patterns)]
#![no_std]

// lots of constants and functions are defined for future use:
#![allow(dead_code)]

// compiler intrinsics like __aeabi_memcpy and __aeabi_uidiv
extern crate compiler_builtins;

// implementations of memset, memcpy... builtins assumed by rust:
// (this feels like a bug. they should be using __aeabi_memset)
extern crate rlibc;

// spinlocks
extern crate spin;

// extern crate volatile;

#[macro_use]
mod console;
mod gpio;
mod limoncello;
mod mailbox;
mod mmio;
mod native;
mod screen;
mod text_display;
mod uart;

use console::CONSOLE;
use screen::screen_init;
use text_display::TEXT_DISPLAY;
use uart::{SERIAL0, UartRate};

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments, _file: &'static str, _line: u32, _column: u32) -> ! {
  loop {}
}

#[no_mangle]
pub extern fn kernel_main(kernel_end: usize) {
  native::enable_cycle_counter();

  screen_init(640, 480, 24).unwrap();

  {
    let mut t = TEXT_DISPLAY.lock();
    t.bg_color = 0xff0000;
    t.fg_color = 0xffffff;
    t.clear();
    t.move_to(0, 33);
  }

  {
    let mut serial = SERIAL0.lock();
    serial.init(UartRate::B115200);
  }

  CONSOLE.lock().set_serial(&SERIAL0);

  let t1 = native::cycle_count();
  print!("CrapOS now booting, please stand by...\n");
  let t2 = native::cycle_count();

  let mem = mailbox::get_memory_info().unwrap();
  print!("Memory: RAM {}MB, GPU {}MB\n", mem.cpu_size >> 20, mem.gpu_size >> 20);
  print!("Kernel reaches {:08x}\n", kernel_end);
  print!("\n");

  print!("© 2018 Gnashers of Insomnia\nFranzösische Straße 1403, Berlin\n");
  print!("cycles: {:0}\n", t2 - t1);
  print!("The meaning of life is {}\n", 42);
  print!("What if I print a line of text that's so long that it will wrap around an 80-column screen?\n");

  loop {
    native::wait_for_event();
    let c = SERIAL0.lock().read_char();
    print!("{} ", c as u32);
  }
}
