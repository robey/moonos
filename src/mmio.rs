/// memory-mapped i/o

use core::intrinsics;

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
