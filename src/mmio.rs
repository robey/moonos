/// memory-mapped i/o

use core::intrinsics;
use native;

pub trait Mmio<Reg: Into<isize>> {
  fn base(&self) -> usize;

  #[inline]
  fn read(&mut self, reg: Reg) -> u32 {
    unsafe { intrinsics::volatile_load((self.base() as *const u8).offset(reg.into()) as *const u32) }
  }

  #[inline]
  fn write(&mut self, reg: Reg, data: u32) {
    unsafe { intrinsics::volatile_store((self.base() as *mut u8).offset(reg.into()) as *mut u32, data) }
  }

  #[inline]
  fn read_atomic(&self, reg: Reg) -> u32 {
    unsafe { intrinsics::atomic_load((self.base() as *const u8).offset(reg.into()) as *const u32) }
  }

  #[inline]
  fn write_atomic(&self, reg: Reg, data: u32) {
    unsafe { intrinsics::atomic_store((self.base() as *mut u8).offset(reg.into()) as *mut u32, data) }
  }
}


use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
pub struct InterruptRegisters {
  pending_basic: AtomicUsize,
}

pub fn plant_registers<A>(base: usize) -> &'static A {
  unsafe { mem::transmute(base) }
  // (r.pending_basic.load(Ordering::Relaxed), r)
}

// map a register struct into MMIO space, for easy access. a closure is used
// so memory barriers can be applied on each side (technically required by
// the BCM docs).
pub fn with_registers<R, A, F>(base: usize, f: F) -> A where F: Fn(&R) -> A {
  native::barrier();
  let rv = f(unsafe { mem::transmute(base) });
  native::barrier();
  rv
}

// enum Reg {
//   PendingBasic = 0,  // 64 - 71
//   PendingGpu1 = 4,   // 0 - 31
//   PendingGpu2 = 8,   // 32 - 63
//   ControlFiq = 12,
//   EnableGpu1 = 16,
//   EnableGpu2 = 20,
//   EnableBasic = 24,
//   DisableGpu1 = 28,
//   DisableGpu2 = 32,
//   DisableBasic = 36,
// }
