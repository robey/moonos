use core::convert::TryFrom;
use mmio::Mmio;
use native;
use optional_callback::OptionalCallback;
use raspi;

pub static INTERRUPTS: Interrupts = Interrupts::new();

pub const IRQ_COUNT: usize = 72;

pub type Callback = fn (irq: usize) -> ();

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

impl TryFrom<usize> for Interrupt {
  type Error = ();

  fn try_from(n: usize) -> Result<Interrupt, ()> {
    match n {
      1 => Ok(Interrupt::Timer1),
      3 => Ok(Interrupt::Timer3),
      _ => Err(()),
    }
  }
}

pub struct Interrupts {
  handlers: [OptionalCallback; IRQ_COUNT],
  clearers: [OptionalCallback; IRQ_COUNT],
}

impl Mmio<Reg> for Interrupts {
  fn base(&self) -> usize { raspi::INTERRUPTS_BASE }
}

impl Interrupts {
  pub const fn new() -> Interrupts {
    Interrupts {
      handlers: [OptionalCallback::new(); IRQ_COUNT],
      clearers: [OptionalCallback::new(); IRQ_COUNT],
    }
  }

  pub fn init(&self) {
    // disable all interrupts
    self.write_atomic(Reg::DisableGpu1, 0xffffffff);
    self.write_atomic(Reg::DisableGpu2, 0xffffffff);
    self.write_atomic(Reg::DisableBasic, 0xffffffff);

    native::enable_interrupts();
  }

  pub fn register(&self, irq: usize, handler: Callback, clearer: Callback) {
    if irq <= IRQ_COUNT {
      if irq < 32 {
        self.write_atomic(Reg::EnableGpu1, 1 << irq);
      } else if irq < 64 {
        self.write_atomic(Reg::EnableGpu2, 1 << (irq - 32));
      } else {
        self.write_atomic(Reg::EnableBasic, 1 << (irq - 64));
      }

      self.handlers[irq].set(Some(handler));
      self.clearers[irq].set(Some(clearer));
    }
  }

  pub fn next_pending_interrupt(&self) -> Option<usize> {
    let pending_gpu1 = self.read_atomic(Reg::PendingGpu1);
    if pending_gpu1 != 0 {
      return Some(pending_gpu1.trailing_zeros() as usize);
    }
    let pending_gpu2 = self.read_atomic(Reg::PendingGpu2);
    if pending_gpu2 != 0 {
      return Some(pending_gpu2.trailing_zeros() as usize + 32);
    }
    let pending_basic = self.read_atomic(Reg::PendingBasic);
    if (pending_basic & 255) != 0 {
      return Some((pending_basic & 255).trailing_zeros() as usize + 64);
    }
    None
  }
}

#[no_mangle]
pub extern fn vector_irq_handler() {
  if let Some(irq) = INTERRUPTS.next_pending_interrupt() {
    if let Some(clearer) = INTERRUPTS.clearers[irq].get() {
      clearer(irq);
      if let Some(handler) = INTERRUPTS.handlers[irq].get() {
        native::enable_interrupts();
        handler(irq);
        native::disable_interrupts();
      }
    }
  }
}
