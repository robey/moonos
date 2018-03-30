// draw on the VGA screen and the serial console at the same time.

use core::fmt;
use spin::Mutex;
use text_display::TEXT_DISPLAY;
use uart::{Uart};

macro_rules! print {
  ($($arg:tt)*) => ($crate::console::print(format_args!($($arg)*)));
}

pub static CONSOLE: Mutex<Console> = Mutex::new(Console::new());

pub struct Console {
  pub serial: Option<&'static Mutex<Uart>>,
}

impl Console {
  pub const fn new() -> Console {
    Console { serial: None }
  }

  pub fn set_serial(&mut self, serial: &'static Mutex<Uart>) {
    self.serial = Some(serial);
  }
}

impl fmt::Write for Console {
  fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
    if let Some(serial) = self.serial {
      serial.lock().write_str(s)?;
    }
    TEXT_DISPLAY.lock().write_str(s)?;
    Ok(())
  }
}

pub fn print(args: fmt::Arguments) {
  // TEXT_DISPLAY.lock().write_fmt(args).unwrap();
  fmt::write(&mut *CONSOLE.lock(), args).unwrap();
}
