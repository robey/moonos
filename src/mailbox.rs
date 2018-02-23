// weird "mailbox" RPC on the raspi, between the cpu and gpu

use mmio::{barrier, Mmio, wait_for_event};

const MAILBOX_BASE: isize = 0x3f00b880;

const STATUS_FULL: u32 = (1 << 31);
const STATUS_EMPTY: u32 = (1 << 30);

const CHAN_PROPERTY: u8 = 8;

const TAG_HW_GET_CPU_MEMORY: u32 = 0x00010005;
const TAG_HW_GET_GPU_MEMORY: u32 = 0x00010006;
const TAG_END: u32 = 0;

enum Reg {
  READ = 0x00,
  STATUS = 0x18,
  WRITE = 0x20,
}

impl Into<isize> for Reg {
  fn into(self) -> isize { self as isize }
}

pub struct Mailbox {
}

impl Mailbox {
  pub fn new() -> Mailbox {
    Mailbox { }
  }

  pub fn read_channel(&self, channel: u8) -> u32 {
    loop {
      while self.read(Reg::STATUS) & STATUS_EMPTY != 0 {
        wait_for_event();
      }
      let data = self.read(Reg::READ);
      if channel == (data & 0xf) as u8 {
        return data >> 4;
      }
    }
  }

  pub fn write_channel(&self, channel: u8, data: u32) {
    barrier();
    while self.read(Reg::STATUS) & STATUS_FULL != 0 {
      wait_for_event();
    }
    self.write(Reg::WRITE, (data << 4) | (channel as u32));
    barrier();
  }
}

impl Mmio<Reg> for Mailbox {
  #[inline]
  fn base(&self) -> *mut u8 { MAILBOX_BASE as *mut u8 }
}

pub fn mailbox() -> Mailbox {
  Mailbox::new()
}


// ----- property channel

const PROPERTY_MAILBOX_BUFFER_SIZE: usize = 32;

const CODE_REQUEST: u32 = 0;
const CODE_RESPONSE_OK: u32 = 0x80000000;
const CODE_RESPONSE_ERROR: u32 = 0x80000001;

#[derive(Debug, PartialEq)]
pub enum PropertyMailboxCode {
  Ok, NoReply, BadReply, Error
}

// the "property" channel takes an address to a buffer of:
// { len: u32, req_or_response: u32, tags... }
#[repr(align(16))]
pub struct PropertyMailbox {
  buffer: [u32; PROPERTY_MAILBOX_BUFFER_SIZE],
  index: usize
}

impl PropertyMailbox {
  pub fn new() -> PropertyMailbox {
    let me = PropertyMailbox { buffer: [0; PROPERTY_MAILBOX_BUFFER_SIZE], index: 2 };
    me
  }

  pub fn add(&mut self, tag: u32, args: &[u32]) {
    self.buffer[self.index] = tag;
    self.buffer[self.index + 1] = (args.len() * 4) as u32;
    self.buffer[self.index + 2] = 0;
    self.index += 3;
    for arg in args {
      self.buffer[self.index] = *arg;
      self.index += 1;
    }
  }

  pub fn write(&mut self, mailbox: &Mailbox) -> PropertyMailboxCode {
    self.buffer[self.index] = TAG_END;
    // pad to align(16)
    self.buffer[self.index + 1] = 0;
    self.buffer[self.index + 2] = 0;
    self.buffer[self.index + 3] = 0;
    self.index += 4;

    self.buffer[0] = ((self.index >> 2) << 4) as u32;
    self.buffer[1] = CODE_REQUEST;

    // what in the name of the rose...
    let data = (&self.buffer as *const [u32] as *const u8 as usize as u32) >> 4;
    mailbox.write_channel(CHAN_PROPERTY, data);
    let response_data = mailbox.read_channel(CHAN_PROPERTY);
    if response_data != data { return PropertyMailboxCode::BadReply }
    decode_error(self.buffer[1])
  }

  pub fn find_tag(&self, tag: u32) -> Option<usize> {
    let mut index = 2;
    if self.buffer[0] < 16 { return None }
    loop {
      if self.buffer[index] == TAG_END { return None }
      if self.buffer[index] == tag { return Some(index) }
      index += 3 + (self.buffer[index + 1] >> 2) as usize;
    }
  }

  pub fn tag_result(&self, tag: u32) -> Option<&[u32]> {
    self.find_tag(tag).and_then(|index| {
      let response = self.buffer[index + 2];
      if response & (1 << 31) == 0 {
        None
      } else {
        let response_size = ((response & 0x7fff_ffff) >> 2) as usize;
        Some(&self.buffer[index + 3 .. index + 3 + response_size])
      }
    })
  }

}

fn decode_error(code: u32) -> PropertyMailboxCode {
  match code {
    CODE_REQUEST => PropertyMailboxCode::NoReply,
    CODE_RESPONSE_OK => PropertyMailboxCode::Ok,
    CODE_RESPONSE_ERROR => PropertyMailboxCode::Error,
    _ => PropertyMailboxCode::BadReply
  }
}

pub struct MemoryInfo {
  pub cpu_base: u32,
  pub cpu_size: u32,
  pub gpu_base: u32,
  pub gpu_size: u32,
}

pub fn get_memory_info() -> Option<MemoryInfo> {
  let mut prop = PropertyMailbox::new();
  // request align(16)
  prop.add(TAG_HW_GET_CPU_MEMORY, &[ 0, 0 ]);
  prop.add(TAG_HW_GET_GPU_MEMORY, &[ 0, 0 ]);
  let rv = prop.write(&mailbox());
  if rv != PropertyMailboxCode::Ok { return None }

  if let Some(&[ cpu_base, cpu_size ]) = prop.tag_result(TAG_HW_GET_CPU_MEMORY) {
    if let Some(&[ gpu_base, gpu_size ]) = prop.tag_result(TAG_HW_GET_GPU_MEMORY) {
      return Some(MemoryInfo { cpu_base, cpu_size, gpu_base, gpu_size })
    }
  }
  None
}
