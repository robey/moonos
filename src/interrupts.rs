use core::convert::TryFrom;
use core::intrinsics;
use mmio::Mmio;
use native;
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
  handlers: [usize; IRQ_COUNT],
  clearers: [usize; IRQ_COUNT],
}

impl Mmio<Reg> for Interrupts {
  fn base(&self) -> usize { raspi::INTERRUPTS_BASE }
}

impl Interrupts {
  pub const fn new() -> Interrupts {
    Interrupts {
      handlers: [0; IRQ_COUNT],
      clearers: [0; IRQ_COUNT],
    }
  }

  pub fn init(&self) {
    // disable all interrupts
    self.write_atomic(Reg::DisableGpu1, 0xffffffff);
    self.write_atomic(Reg::DisableGpu2, 0xffffffff);
    self.write_atomic(Reg::DisableBasic, 0xffffffff);

    native::enable_interrupts();
  }

  fn get_callback(&self, vector: &[usize], irq: usize) -> Option<Callback> {
    let ptr = unsafe { intrinsics::volatile_load(&vector[irq] as *const usize) as *const usize };
    if ptr.is_null() {
      None
    } else {
      Some(unsafe { *(&ptr as *const *const usize as *const Callback) })
    }
  }

  fn set_callback(&self, vector: &[usize], irq: usize, callback: Callback) {
    let ptr = callback as *const Callback;
    // FIXME gross
    unsafe { intrinsics::atomic_store(&vector[irq] as *const usize as usize as *mut usize, ptr as usize); }
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

      self.set_callback(&self.handlers, irq, handler);
      self.set_callback(&self.clearers, irq, clearer);
    }
  }

  pub fn next_pending_interrupt(&self) -> Option<u32> {
    let pending_gpu1 = self.read_atomic(Reg::PendingGpu1);
    if pending_gpu1 != 0 {
      return Some(pending_gpu1.trailing_zeros());
    }
    let pending_gpu2 = self.read_atomic(Reg::PendingGpu2);
    if pending_gpu2 != 0 {
      return Some(pending_gpu2.trailing_zeros() + 32);
    }
    let pending_basic = self.read_atomic(Reg::PendingBasic);
    if (pending_basic & 255) != 0 {
      return Some((pending_basic & 255).trailing_zeros() + 64);
    }
    None
  }
}

#[no_mangle]
pub extern fn vector_irq_handler() {
  if let Some(irq) = INTERRUPTS.next_pending_interrupt() {
    if let Some(handler) = INTERRUPTS.get_callback(&INTERRUPTS.handlers, irq as usize) {
      let clearer = INTERRUPTS.get_callback(&INTERRUPTS.clearers, irq as usize);
      clearer.map(|c| c(irq as usize));
      native::enable_interrupts();
      handler(irq as usize);
      native::disable_interrupts();
    }
  }
}
