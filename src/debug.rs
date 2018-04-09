use core::slice;


fn print_hex_line(address: usize, skip_front: usize, skip_back: usize, as_bytes: bool) {
  let ptr = unsafe { slice::from_raw_parts(address as *const u8, 16) };
  let word_ptr = unsafe { slice::from_raw_parts(address as *const u32, 16) };

  print!("{:08x}:  ", address as u32);

  let mut i = 0;
  while i < 16 {
    if i == 8 && as_bytes { print!(" "); }
    if i < skip_front || i >= 16 - skip_back {
      if as_bytes {
        print!("   ");
      } else {
        print!("         ");
      }
    } else {
      if as_bytes {
        print!("{:02x} ", ptr[i]);
      } else {
        print!("{:08x} ", word_ptr[i >> 2]);
      }
    }
    i += if as_bytes { 1 } else { 4 };
  }

  print!("  ");

  for i in 0..16 {
    if i == 8 { print!(" "); }
    if i < skip_front || i >= 16 - skip_back {
      print!(" ");
    } else {
      print!("{}", if ptr[i] >= 0x20 && ptr[i] <= 0x7e { ptr[i] as char } else { '.' });
    }
  }

  print!("\n");
}

pub fn dump_memory(address_start: usize, address_end: usize, as_bytes: bool) {
  let mut address = address_start & 0xffff_fff0;
  let address_last = (address_end + 7) & 0xffff_fff0;
  while address < address_last {
    let skip_front = if address < address_start { address_start - address } else { 0 };
    let skip_back = if address + 16 > address_end { address + 16 - address_end } else { 0 };
    print_hex_line(address, skip_front, skip_back, as_bytes);
    address += 16;
  }
}

pub fn dump_registers(saved_registers: usize) {
  let regs = unsafe { slice::from_raw_parts((saved_registers + 4) as *const u32, 16) };
  let psr: u32 = unsafe { *(saved_registers as *const u32) };
  print!(" r0: {:08x}   r4: {:08x}   r8: {:08x}   ip: {:08x}\n", regs[0], regs[4], regs[8], regs[12]);
  print!(" r1: {:08x}   r5: {:08x}   r9: {:08x}   sp: {:08x}\n", regs[1], regs[5], regs[9], regs[13]);
  print!(" r2: {:08x}   r6: {:08x}  r10: {:08x}   lr: {:08x}\n", regs[2], regs[6], regs[10], regs[14]);
  print!(" r3: {:08x}   r7: {:08x}  r11: {:08x}   pc: {:08x}  psr: {:08x}\n", regs[3], regs[7], regs[11], regs[15], psr);
}

pub fn dump(saved_registers: usize) {
  print!("----- registers\n");
  dump_registers(saved_registers);
  let sp = unsafe { *((saved_registers + 14 * 4) as *const u32) } as usize;
  print!("----- stack\n");
  dump_memory(sp, sp + 128, false);
}
