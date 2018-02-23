/// memory-mapped i/o

use core::intrinsics;

#[inline]
pub fn delay(cycles: usize) {
  let mut _n = cycles;
  unsafe {
    asm!(
      "
      ${:private}_delay_${:uid}:
        subs $0, $0, #1
        bne ${:private}_delay_${:uid}
      "
      : "=r"(_n)    // outputs
      : "0"(_n)     // inputs
      : "cc"        // clobbers
      : "volatile"  // options
    );
  }
}

// "data synchronization barrier"
#[inline]
pub fn barrier() {
  unsafe { asm!("dsb" ::: "memory" : "volatile") }
}

#[inline]
pub fn wait_for_event() {
  unsafe { asm!("wfe" :::: "volatile") }
}

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
