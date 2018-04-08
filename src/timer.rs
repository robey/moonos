use interrupts::{Interrupt, INTERRUPTS};
use mmio::Mmio;
use raspi;
use spinlock::Mutex;

pub static TIMER: Mutex<Timer> = Mutex::new(Timer::new());

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

  pub fn init(&mut self) {
    // only 1 and 3 are available, because the other 2 are used by the GPU.
    INTERRUPTS.lock().register(Interrupt::Timer1 as usize, handle_interrupt, clear_interrupt);
    INTERRUPTS.lock().register(Interrupt::Timer3 as usize, handle_interrupt, clear_interrupt);
  }

  pub fn clear(&mut self) {
    let x = self.read(Reg::Control) | (1 << 1);
    self.write(Reg::Control, x);
  }

  // we only set timer3.
  pub fn set(&mut self, usec: u32) {
    let counter = self.read(Reg::CounterLow);
    self.write(Reg::Timer3, counter.wrapping_add(usec));
    self.clear();
  }

  pub fn get(&mut self) -> u64 {
    // FIXME account for rollover while reading.
    return (self.read(Reg::CounterLow) as u64) | (self.read(Reg::CounterHigh) as u64) << 32;
  }

  pub fn get_next(&mut self) -> u32 {
    self.read(Reg::Control)
  }
}

pub fn handle_interrupt(_n: usize) {
  print!("timer!\n");
}

pub fn clear_interrupt(_n: usize) {
  TIMER.lock().clear();
}
