use mmio::Mmio;
use native;
use raspi;
use spinlock::Mutex;

pub static INTERRUPTS: Mutex<Interrupts> = Mutex::new(Interrupts::new());

pub const IRQ_COUNT: usize = 72;

pub trait InterruptHandler : Sync {
  fn handle_interrupt(&self, interrupt: usize) -> ();
  fn clear_interrupt(&self, interrupt: usize) -> ();
}

pub enum Interrupt {
  Timer = 1,
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
  handlers: [Option<&'static InterruptHandler>; IRQ_COUNT],
}

impl Mmio<Reg> for Interrupts {
  fn base(&self) -> usize { raspi::INTERRUPTS_BASE }
}

impl Interrupts {
  pub const fn new() -> Interrupts {
    Interrupts { handlers: [None; IRQ_COUNT] }
  }

  pub fn init(&mut self) {
    // disable all interrupts
    self.write(Reg::DisableGpu1, 0xffffffff);
    self.write(Reg::DisableGpu2, 0xffffffff);
    self.write(Reg::DisableBasic, 0xffffffff);

    native::enable_interrupts();
  }

  pub fn register(&mut self, interrupt: Interrupt, handler: &'static InterruptHandler) {
    let n = interrupt as usize;
    if n <= IRQ_COUNT {
      if n < 32 {
        let x = self.read(Reg::EnableGpu1) | (1 << n);
        self.write(Reg::EnableGpu1, x);
      } else if n < 64 {
        let x = self.read(Reg::EnableGpu2) | (1 << (n - 32));
        self.write(Reg::EnableGpu2, x);
      } else {
        let x = self.read(Reg::EnableBasic) | (1 << (n - 64));
        self.write(Reg::EnableBasic, x);
      }
      self.handlers[n] = Some(handler);
    }
  }

  fn next_pending_interrupt(&mut self) -> Option<u32> {
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
  let mut i = INTERRUPTS.lock();
  if let Some(interrupt) = i.next_pending_interrupt() {
    if let Some(handler) = i.handlers[interrupt as usize] {
      handler.clear_interrupt(interrupt as usize);
      native::enable_interrupts();
      handler.handle_interrupt(interrupt as usize);
      native::disable_interrupts();
    }
  }
}
