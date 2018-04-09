use core::convert::{TryFrom};
use interrupts::{Interrupt, INTERRUPTS};
use mmio::Mmio;
use raspi;

pub static TIMER: Timer = Timer::new();

enum Reg {
  Control = 0,
  CounterLow = 4,
  CounterHigh = 8,
  Timer0 = 12,
  Timer1 = 16,
  Timer2 = 20,
  Timer3 = 24,
}

impl Into<isize> for Reg {
  fn into(self) -> isize { self as isize }
}

pub struct Timer {

}

impl Mmio<Reg> for Timer {
  fn base(&self) -> usize { raspi::TIMER_BASE }
}

impl Timer {
  pub const fn new() -> Timer {
    Timer { }
  }

  pub fn init(&self) {
    // only 1 and 3 are available, because the other 2 are used by the GPU.
    self.clear(Interrupt::Timer1);
    self.clear(Interrupt::Timer3);
    INTERRUPTS.register(Interrupt::Timer1 as usize, handle_interrupt, clear_interrupt);
    INTERRUPTS.register(Interrupt::Timer3 as usize, handle_interrupt, clear_interrupt);
  }

  fn clear(&self, timer: Interrupt) {
    self.write_atomic(Reg::Control, 1 << (timer as usize));
  }

  // we only set timer3.
  pub fn set(&self, usec: u32) {
    let counter = self.read_atomic(Reg::CounterLow);
    self.write_atomic(Reg::Timer3, counter.wrapping_add(usec));
  }

  pub fn get(&self) -> u64 {
    // read the high word before & after the low, to make sure we don't get
    // a split value during rollover. (think: 1_ffffffff to 2_00000000,
    // returning 1_00000000)
    loop {
      let initial_high = self.read_atomic(Reg::CounterHigh) as u64;
      let low = self.read_atomic(Reg::CounterLow) as u64;
      if initial_high == self.read_atomic(Reg::CounterHigh) as u64 {
        return (initial_high << 32) | low;
      }
    }
  }

  pub fn get_next(&self) -> u32 {
    self.read_atomic(Reg::Control)
  }
}

pub fn handle_interrupt(_n: usize) {
  print!("timer!\n");
}

pub fn clear_interrupt(irq: usize) {
  print!("clear!");
  match Interrupt::try_from(irq) {
    Ok(i) => TIMER.clear(i),
    Err(_) => ()
  };
}
