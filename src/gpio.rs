use mmio::Mmio;
use native;
use raspi;
use spinlock::Mutex;

pub static GPIO: Mutex<Gpio> = Mutex::new(Gpio::new());

pub enum PudMode {
  Off = 0,
  Pulldown = 1,
  Pullup = 2
}

enum Reg {
  // pull up/down:
  GPPUD = 0x94,

  // clock the pull up/down into pins:
  GPPUDCLK0 = 0x98,
  GPPUDCLK1 = 0x9c,
}

impl Into<isize> for Reg {
  fn into(self) -> isize { self as isize }
}

// GPIO controls
pub struct Gpio {
}

impl Gpio {
  pub const fn new() -> Gpio {
    Gpio { }
  }

  pub fn configure_pins(&mut self, mode: PudMode, pins: &[usize]) {
    let mut mask0: u32 = 0;
    let mut mask1: u32 = 0;
    for i in 0..pins.len() {
      let pin = pins[i];
      if pin < 32 {
        mask0 |= 1 << pin;
      } else {
        mask1 |= 1 << (pin - 32);
      }
    }

    self.write(Reg::GPPUD, mode as u32);
    native::delay_cycles(150);
    self.write(Reg::GPPUDCLK0, mask0);
    self.write(Reg::GPPUDCLK1, mask1);
    native::delay_cycles(150);
    self.write(Reg::GPPUDCLK0, 0);
    self.write(Reg::GPPUDCLK1, 0);
  }
}

impl Mmio<Reg> for Gpio {
  fn base(&self) -> usize { raspi::GPIO_BASE }
}
