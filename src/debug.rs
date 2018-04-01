use core::fmt;
use core::slice;
use core::str;

struct Line {
  buffer: [u8; 80],
  pos: usize,
}

impl Line {
  pub fn new() -> Line {
    Line { buffer: [0; 80], pos: 0 }
  }

  pub fn str(&self) -> &str {
    str::from_utf8(&self.buffer[0..self.pos]).unwrap()
  }
}

impl fmt::Write for Line {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for b in s.bytes() {
      if self.pos >= 80 { return Err(fmt::Error) };
      self.buffer[self.pos] = b;
      self.pos += 1;
    }
    Ok(())
  }
}


fn generate_hex_line(address: usize, skip_front: usize, skip_back: usize) -> Result<Line, fmt::Error> {
  use core::fmt::Write;
  let mut line = Line::new();
  let ptr = unsafe { slice::from_raw_parts(address as *const u8, 16) };

  write!(&mut line, "{:04x}_{:04x}: ", address >> 16, address & 0xffff)?;

  for i in 0..16 {
    if i == 8 { write!(&mut line, " ")?; }
    if i < skip_front || i >= 16 - skip_back {
      write!(&mut line, "   ")?;
    } else {
      write!(&mut line, "{:02x} ", ptr[i])?;
    }
  }

  write!(&mut line, "  ")?;

  for i in 0..16 {
    if i == 8 { write!(&mut line, " ")?; }
    if i < skip_front || i >= 16 - skip_back {
      write!(&mut line, " ")?;
    } else {
      write!(&mut line, "{}", if ptr[i] >= 0x20 && ptr[i] <= 0x7e { ptr[i] as char } else { '.' })?;
    }
  }

  Ok(line)
}

pub fn dump_memory(address_start: usize, address_end: usize) {
  let mut address = address_start & 0xffff_fff0;
  let address_last = (address_end + 7) & 0xffff_fff0;
  while address < address_last {
    let skip_front = if address < address_start { address_start - address } else { 0 };
    let skip_back = if address + 16 > address_end { address + 16 - address_end } else { 0 };
    if let Ok(line) = generate_hex_line(address, skip_front, skip_back) {
      print!("{}\n", line.str());
    }
    address += 16;
  }
}
