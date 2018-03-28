
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

pub unsafe fn copy_memory(mut dest: *mut u8, mut source: *const u8, mut count: usize) {
  if (dest as usize & 15) == (source as usize & 15) {
    // matched alignment

    // copy bytes until we're word-aligned
    while count > 0 && (dest as usize & 3) != 0 {
      *dest = *source;
      dest = dest.offset(1);
      source = source.offset(1);
      count -= 1;
    }

    while count >= 16 {
      asm!(
        "
        ldmia r1!, {r4-r7}
        stmia r0!, {r4-r7}
        "
        : "={r0}"(dest), "={r1}"(source)
        : "{r0}"(dest), "{r1}"(source)
        : "r4", "r5", "r6", "r7", "memory"
        : "volatile"
      );
      count -= 16;
    }

    while count >= 4 {
      asm!(
        "
        ldr r4, [r1], #4
        str r4, [r0], #4
        "
        : "={r0}"(dest), "={r1}"(source)
        : "{r0}"(dest), "{r1}"(source)
        : "r4", "memory"
        : "volatile"
      );
      count -= 4;
    }
  }

  // remaining bytes
  while count > 0 {
    *dest = *source;
    dest = dest.offset(1);
    source = source.offset(1);
    count -= 1;
  }
}
