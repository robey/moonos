use core::convert::TryFrom;
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

pub struct InterruptRegisters {
  pending_basic: u32,  // 64 - 71
  pending_gpu_1: u32,  // 0 - 31
  pending_gpu_2: u32,  // 32 - 63
  control_fiq: u32,
  enable_gpu_1: u32,
  enable_gpu_2: u32,
  enable_basic: u32,
  disable_gpu_1: u32,
  disable_gpu_2: u32,
  disable_basic: u32,
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

impl Interrupts {
  pub const fn new() -> Interrupts {
    Interrupts {
      handlers: [OptionalCallback::new(); IRQ_COUNT],
      clearers: [OptionalCallback::new(); IRQ_COUNT],
    }
  }

  pub fn init(&self) {
    // disable all interrupts
    native::with_registers(raspi::INTERRUPTS_BASE, |r: &mut InterruptRegisters| {
      r.disable_gpu_1 = 0xffffffff;
      r.disable_gpu_2 = 0xffffffff;
      r.disable_basic = 0xffffffff;
    });
    native::enable_interrupts();
  }

  pub fn register(&self, irq: usize, handler: Callback, clearer: Callback) {
    if irq <= IRQ_COUNT {
      native::with_registers(raspi::INTERRUPTS_BASE, |r: &mut InterruptRegisters| {
        if irq < 32 {
          r.enable_gpu_1 = 1 << irq;
        } else if irq < 64 {
          r.enable_gpu_2 = 1 << (irq - 32);
        } else {
          r.enable_basic = 1 << (irq - 64);
        }
      });

      self.handlers[irq].set(Some(handler));
      self.clearers[irq].set(Some(clearer));
    }
  }

  pub fn next_pending_interrupt(&self) -> Option<usize> {
    native::with_registers(raspi::INTERRUPTS_BASE, |r: &mut InterruptRegisters| {
      let pending_gpu_1 = r.pending_gpu_1;
      if pending_gpu_1 != 0 {
        return Some(pending_gpu_1.trailing_zeros() as usize);
      }
      let pending_gpu_2 = r.pending_gpu_2;
      if pending_gpu_2 != 0 {
        return Some(pending_gpu_2.trailing_zeros() as usize + 32);
      }
      let pending_basic = r.pending_basic;
      if (pending_basic & 255) != 0 {
        return Some((pending_basic & 255).trailing_zeros() as usize + 64);
      }
      None
    })
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
