use mmio::Mmio;
use native;
use raspi;
use spinlock::Mutex;

pub static INTERRUPTS: Mutex<Interrupts> = Mutex::new(Interrupts::new());

pub const IRQ_COUNT: usize = 72;

pub type InterruptHandler = fn (interrupt: usize) -> ();
pub type InterruptClearer = fn (interrupt: usize) -> ();

pub enum Interrupt {
  Timer1 = 1,
  Timer3 = 3,
}

// offsets into the memory-mapped uart base:
enum Reg {
  PendingBasic = 0,  // 64 - 71
  PendingGpu1 = 4,   // 0 - 31
  PendingGpu2 = 8,   // 32 - 63
  ControlFiq = 12,
  EnableGpu1 = 16,
  EnableGpu2 = 20,
  EnableBasic = 24,
  DisableGpu1 = 28,
  DisableGpu2 = 32,
  DisableBasic = 36,
}

impl Into<isize> for Reg {
  fn into(self) -> isize { self as isize }
}


pub struct Interrupts {
  handlers: [Option<InterruptHandler>; IRQ_COUNT],
  clearers: [Option<InterruptClearer>; IRQ_COUNT],
}

impl Mmio<Reg> for Interrupts {
  fn base(&self) -> usize { raspi::INTERRUPTS_BASE }
}

impl Interrupts {
  pub const fn new() -> Interrupts {
    Interrupts { handlers: [None; IRQ_COUNT], clearers: [None; IRQ_COUNT] }
  }

  pub fn init(&mut self) {
    // disable all interrupts
    self.write(Reg::DisableGpu1, 0xffffffff);
    self.write(Reg::DisableGpu2, 0xffffffff);
    self.write(Reg::DisableBasic, 0xffffffff);

    native::enable_interrupts();
  }

  pub fn register(&mut self, interrupt: usize, handler: InterruptHandler, clearer: InterruptClearer) {
    if interrupt <= IRQ_COUNT {
      if interrupt < 32 {
        self.write(Reg::EnableGpu1, 1 << interrupt);
      } else if interrupt < 64 {
        self.write(Reg::EnableGpu2, 1 << (interrupt - 32));
      } else {
        self.write(Reg::EnableBasic, 1 << (interrupt - 64));
      }
      self.handlers[interrupt] = Some(handler);
      self.clearers[interrupt] = Some(clearer);
    }
  }

  pub fn next_pending_interrupt(&mut self) -> Option<u32> {
    let pending_gpu1 = self.read(Reg::PendingGpu1);
    if pending_gpu1 != 0 {
      return Some(pending_gpu1.trailing_zeros());
    }
    let pending_gpu2 = self.read(Reg::PendingGpu2);
    if pending_gpu2 != 0 {
      return Some(pending_gpu2.trailing_zeros() + 32);
    }
    let pending_basic = self.read(Reg::PendingBasic);
    if (pending_basic & 255) != 0 {
      return Some((pending_basic & 255).trailing_zeros() + 64);
    }
    None
  }
}

#[no_mangle]
pub extern fn vector_irq_handler() {
  // FIXME: don't compete with kernel for interrupt lock. bad bad.
  let mut i = INTERRUPTS.lock();
  if let Some(interrupt) = i.next_pending_interrupt() {
    if let Some(handler) = i.handlers[interrupt as usize] {
      i.clearers[interrupt as usize].map(|f| f(interrupt as usize));
      native::enable_interrupts();
      handler(interrupt as usize);
      native::disable_interrupts();
    }
  }
}
