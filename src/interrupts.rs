use core::convert::TryFrom;
use core::sync::atomic::{AtomicUsize, Ordering};
use mmio::{Mmio, with_registers};
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
  pending_basic: AtomicUsize,  // 64 - 71
  pending_gpu_1: AtomicUsize,  // 0 - 31
  pending_gpu_2: AtomicUsize,  // 32 - 63
  control_fiq: AtomicUsize,
  enable_gpu_1: AtomicUsize,
  enable_gpu_2: AtomicUsize,
  enable_basic: AtomicUsize,
  disable_gpu_1: AtomicUsize,
  disable_gpu_2: AtomicUsize,
  disable_basic: AtomicUsize,
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
    with_registers(raspi::INTERRUPTS_BASE, |r: &InterruptRegisters| {
      r.disable_gpu_1.store(0xffffffff, Ordering::Relaxed);
      r.disable_gpu_2.store(0xffffffff, Ordering::Relaxed);
      r.disable_basic.store(0xffffffff, Ordering::Relaxed);
    });
    native::enable_interrupts();
  }

  pub fn register(&self, irq: usize, handler: Callback, clearer: Callback) {
    if irq <= IRQ_COUNT {
      with_registers(raspi::INTERRUPTS_BASE, |r: &InterruptRegisters| {
        if irq < 32 {
          r.enable_gpu_1.store(1 << irq, Ordering::Relaxed);
        } else if irq < 64 {
          r.enable_gpu_2.store(1 << (irq - 32), Ordering::Relaxed);
        } else {
          r.enable_basic.store(1 << (irq - 64), Ordering::Relaxed);
        }
      });

      self.handlers[irq].set(Some(handler));
      self.clearers[irq].set(Some(clearer));
    }
  }

  pub fn next_pending_interrupt(&self) -> Option<usize> {
    with_registers(raspi::INTERRUPTS_BASE, |r: &InterruptRegisters| {
      let pending_gpu_1 = r.pending_gpu_1.load(Ordering::Relaxed);
      if pending_gpu_1 != 0 {
        return Some(pending_gpu_1.trailing_zeros() as usize);
      }
      let pending_gpu_2 = r.pending_gpu_2.load(Ordering::Relaxed);
      if pending_gpu_2 != 0 {
        return Some(pending_gpu_2.trailing_zeros() as usize + 32);
      }
      let pending_basic = r.pending_basic.load(Ordering::Relaxed);;
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
