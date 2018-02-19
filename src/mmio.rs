/// memory-mapped i/o

use core::intrinsics;

// raspi 2, 3:
const GPIO_BASE: isize = 0x3f200000;

pub enum PudMode {
  Off = 0,
  // Pulldown = 1,
  // Pullup = 2
}

// pull up/down:
const REG_GPPUD: isize = 0x94;

// clock the pull up/down into pins:
const REG_GPPUDCLK0: isize = 0x98;
const REG_GPPUDCLK1: isize = 0x9c;

#[inline]
fn delay(cycles: usize) {
  let mut _n = cycles;
  unsafe {
    asm!(
      "
      ${:private}_delay_${:uid}:
        subs $0, $0, #1
        bne ${:private}_delay_${:uid}
      "
      : "=r"(_n)     // outputs
      : "0"(_n)      // inputs
      : "cc"        // clobbers
      : "volatile"  // options
    );
  }
}

pub trait Mmio {
  fn base(&self) -> *mut u8;

  #[inline]
  fn read(&self, offset: isize) -> u32 {
    unsafe { intrinsics::volatile_load(self.base().offset(offset) as *const u32) }
  }

  #[inline]
  fn write(&self, offset: isize, data: u32) {
    unsafe { intrinsics::volatile_store(self.base().offset(offset) as *mut u32, data) }
  }
}

// GPIO controls
pub struct Gpio {
}

impl Gpio {
  pub fn new() -> Gpio {
    Gpio { }
  }

  pub fn configure_pins(&self, mode: PudMode, pins: &[usize]) {
    let mut mask0: u32 = 0;
    let mut mask1: u32 = 0;
    for i in 0..pins.len() {
      let pin = pins[i];
      if pin < 32 {
        mask0 |= 1 << (pin as u8);
      } else {
        mask1 |= 1 << (pin - 32);
      }
    }

    self.write(REG_GPPUD, mode as u32);
    delay(150);
    self.write(REG_GPPUDCLK0, mask0);
    self.write(REG_GPPUDCLK1, mask1);
    delay(150);
    self.write(REG_GPPUDCLK0, 0);
    self.write(REG_GPPUDCLK1, 0);
  }
}

impl Mmio for Gpio {
  #[inline]
  fn base(&self) -> *mut u8 { GPIO_BASE as *mut u8 }
}

// pub static  _gpio: Option<Gpio> = None;

pub fn gpio() -> Gpio {
  // unsafe { _gpio.get_or_insert_with(|| Gpio::new()) }
  Gpio::new()
}
