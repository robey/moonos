
// asm(text : outputs : inputs : clobbers : options)

enum ProcessorMode {
  User = 0x10,
  Fiq = 0x11,
  Irq = 0x12,
  Supervisor = 0x13,
  Abort = 0x17,
  Undefined = 0x1b,
  System = 0x1f,
}

enum ProcessorFlags {
  Thumb = 1 << 5,
  DisableFiq = 1 << 6,
  DisableIrq = 1 << 7,
  Abort = 1 << 8,
  Endian = 1 << 9,
  StickyOverflow = 1 << 27,
  Overflow = 1 << 28,
  Carry = 1 << 29,
  Zero = 1 << 30,
  Negative = 1 << 31,
}

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

pub fn halt() -> ! {
  loop { wait_for_interrupt(); };
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

#[inline]
pub fn interrupts_enabled() -> bool {
  let mut _rv: usize = 0;
  unsafe { asm!("mrs $0, cpsr" : "=r"(_rv) ::: "volatile") };
  _rv & ProcessorFlags::DisableIrq as usize == 0
}

#[inline]
pub fn enable_interrupts() {
  if !interrupts_enabled() {
    unsafe { asm!("cpsie i" :::: "volatile") };
  }
}

#[inline]
pub fn disable_interrupts() {
  if interrupts_enabled() {
    unsafe { asm!("cpsid i" :::: "volatile") };
  }
}

#[inline]
pub fn syscall(syscall_number: usize, param1: usize, param2: usize, param3: usize, param4: usize) -> usize {
  let mut _rv: usize;
  unsafe {
    asm!(
      "svc #0"
      : "={r0}"(_rv)
      : "{r4}"(syscall_number), "{r0}"(param1), "{r1}"(param2), "{r2}"(param3), "{r3}"(param4)
      : "r0", "r1", "r2", "r3", "r12", "cc"
      : "volatile"
    );
  }
  _rv
}

pub unsafe fn copy_memory(mut dest: *mut u8, mut source: *const u8, mut count: usize) {
  if (source as usize) < (dest as usize) && (source as usize + count) > (dest as usize) {
    // overlapping, with dest after source! must copy backwards.
    copy_memory_backwards(dest, source, count);
    return;
  }

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

pub unsafe fn copy_memory_backwards(mut dest: *mut u8, mut source: *const u8, mut count: usize) {
  dest = dest.offset(count as isize);
  source = source.offset(count as isize);

  if (dest as usize & 15) == (source as usize & 15) {
    // matched alignment

    // copy bytes until we're word-aligned
    while count > 0 && (dest as usize & 3) != 0 {
      dest = dest.offset(-1);
      source = source.offset(-1);
      count -= 1;
      *dest = *source;
    }

    while count >= 16 {
      asm!(
        "
        ldmdb r1!, {r4-r7}
        stmdb r0!, {r4-r7}
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
        ldr r4, [r1, #-4]!
        str r4, [r0, #-4]!
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
    dest = dest.offset(-1);
    source = source.offset(-1);
    count -= 1;
    *dest = *source;
  }
}
