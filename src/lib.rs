#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(slice_patterns)]
#![feature(try_from)]
#![no_std]

// lots of constants and functions are defined for future use:
#![allow(dead_code)]

// compiler intrinsics like __aeabi_memcpy and __aeabi_uidiv
extern crate compiler_builtins;

// implementations of memset, memcpy... builtins assumed by rust:
// (this feels like a bug. they should be using __aeabi_memset)
extern crate rlibc;

#[macro_use]
mod console;
mod debug;
mod gpio;
pub mod interrupts;
mod limoncello;
mod mailbox;
mod mmio;
mod native;
mod raspi;
mod screen;
mod spinlock;
mod text_display;
mod timer;
mod uart;
pub mod vectors;

use console::CONSOLE;
use screen::screen_init;
use text_display::TEXT_DISPLAY;
use uart::{SERIAL0, UartRate};

const CONSOLE_BG: u32 = 0x006000;
const CONSOLE_FG: u32 = 0xffffff;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments, _file: &'static str, _line: u32, _column: u32) -> ! {
  native::halt();
}

#[no_mangle]
pub extern fn kernel_main(kernel_end: usize, exception_vector: usize) {
  native::enable_cycle_counter();

  // first, set up the display & serial console, so we can log.
  SERIAL0.lock().init(UartRate::B115200);
  if screen_init(640, 480, 24).is_err() {
    if SERIAL0.lock().write_str("ERR\n").is_err() {
      // can't do anything.
    }
    native::halt();
  }

  TEXT_DISPLAY.lock().init(CONSOLE_FG, CONSOLE_BG);
  CONSOLE.lock().set_serial(&SERIAL0);

  print!("CrapOS booting...\n");

  // copy 16 words from exception_vector to 0x0, where ARM expects.
  // (these are defined in vectors.S)
  unsafe {
    native::copy_memory(0 as *mut u8, exception_vector as *const u8, 16 * 8);
  }
  interrupts::INTERRUPTS.lock().init();

  print!("rv = {}\n", native::syscall(0, 23, 0, 0));

  let t1 = native::cycle_count();
  let mem = mailbox::get_memory_info().unwrap();
  let t2 = native::cycle_count();
  print!("Memory: RAM {}MB, GPU {}MB\n", mem.cpu_size >> 20, mem.gpu_size >> 20);
  print!("Kernel reaches {:08x}\n", kernel_end);
  print!("Exception vector: {:08x}\n", exception_vector);
  print!("\n");

  print!("© 2018 Gnashers of Insomnia\nFranzösische Straße 1403, Berlin\n");
  print!("cycles: {:0}\n", t2 - t1);
  print!("The meaning of life is {}\n", 42);
  print!("What if I print a line of text that's so long that it will wrap around an 80-column screen?\n");

  timer::TIMER.lock().init();
  timer::TIMER.lock().set(1000000);
  print!("TIMER {}\n", timer::TIMER.lock().get());

  loop {
    native::wait_for_event();
  //  print!("TIMER {}\n", timer::TIMER.lock().get());
  //  print!("next {}\n", timer::TIMER.lock().get_next());
  //  print!("next {:?}\n", interrupts::INTERRUPTS.lock().next_pending_interrupt());

     // let c = SERIAL0.lock().read_char();
    // print!("{} ", c as u32);
  }
}
