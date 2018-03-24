/// memory-mapped i/o

use core::intrinsics;

pub trait Mmio<Reg: Into<isize>> {
  fn base(&self) -> *mut u8;

  #[inline]
  fn read(&self, reg: Reg) -> u32 {
    unsafe { intrinsics::volatile_load(self.base().offset(reg.into()) as *const u32) }
  }

  #[inline]
  fn write(&self, reg: Reg, data: u32) {
    unsafe { intrinsics::volatile_store(self.base().offset(reg.into()) as *mut u32, data) }
  }
}
