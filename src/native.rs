
// asm(text : outputs : inputs : clobbers : options)

// "data synchronization barrier"
#[inline]
pub fn barrier() {
  unsafe { asm!("dsb" ::: "memory" : "volatile") };
}

// halt until interrupt or `sev` (wakeup for spinlocks)
#[inline]
pub fn wait_for_event() {
  unsafe { asm!("wfe" :::: "volatile") };
}

// halt until interrupt
#[inline]
pub fn wait_for_interrupt() {
  unsafe { asm!("wfi" :::: "volatile") };
}

// the cycle counter has to be explicitly turned on.
pub fn enable_cycle_counter() {
  let mut _r: u32 = 1;
  unsafe {
    // pmuserenr = 1
    asm!("mcr p15, 0, $0, c9, c14, 0" :: "r"(_r) :: "volatile");
    // pmcr:0 = 1, pmcr:2 = 1, pmcr:3 = 1
    _r = (1 << 3) | (1 << 2) | (1 << 0);
    asm!("mcr p15, 0, $0, c9, c12, 0" :: "r"(_r) :: "volatile");
    // pmcntenset:31 = 1
    _r = 1 << 31;
    asm!("mcr p15, 0, $0, c9, c12, 1" :: "r"(_r) :: "volatile");
  }
}

#[inline]
pub fn cycle_count() -> u64 {
  let mut _r: u32 = 0;
  unsafe { asm!("mrc p15, 0, $0, c9, c13, 0" : "=r"(_r) ::: "volatile") };
  return (_r as u64) << 6;
}

#[inline]
pub fn delay_cycles(cycles: u32) {
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
