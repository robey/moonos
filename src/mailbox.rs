// weird "mailbox" RPC on the raspi, between the cpu and gpu

use mmio::{barrier, Mmio, wait_for_event};

const MAILBOX_BASE: isize = 0x3f00b880;

const REG_READ: isize = 0x00;
const REG_STATUS: isize = 0x18;
const REG_WRITE: isize = 0x20;

const STATUS_FULL: u32 = (1 << 31);
const STATUS_EMPTY: u32 = (1 << 30);

const CHAN_PROPERTY: u8 = 8;

const MAILBOX_BUFFER_SIZE: usize = 128;
static mut MAILBOX_BUFFER: [u32; MAILBOX_BUFFER_SIZE] = [0; MAILBOX_BUFFER_SIZE];

const TAG_FB_SET_SIZE: u32 = 0x00048003;
const TAG_FB_SET_VIRTUAL_SIZE: u32 = 0x00048004;
const TAG_FB_SET_DEPTH: u32 = 0x00048005;
const TAG_END: u32 = 0;

// need the mailbox buffer to be aligned to 16, which rust can't currently do
fn align_usize(n: usize) -> usize {
  ((n + 15) >> 4) << 4
}
fn align_ptr<T>(p: *mut T) -> *mut T {
  align_usize(p as usize) as *mut T
}
fn buffer_aligned() -> &'static mut [u32; MAILBOX_BUFFER_SIZE] {
  // don't judge me!
  unsafe { &mut *align_ptr(&mut MAILBOX_BUFFER as *mut [u32; MAILBOX_BUFFER_SIZE]) as &mut [u32; MAILBOX_BUFFER_SIZE] }
}

pub struct Mailbox {
}

impl Mailbox {
  pub fn new() -> Mailbox {
    Mailbox { }
  }

  pub fn read_channel(&self, channel: u8) -> u32 {
    loop {
      while self.read(REG_STATUS) & STATUS_EMPTY != 0 {
        wait_for_event();
      }
      let data = self.read(REG_READ);
      if channel == (data & 0xf) as u8 {
        return data >> 4;
      }
    }
  }

  pub fn write_channel(&self, channel: u8, data: u32) {
    barrier();
    while self.read(REG_STATUS) & STATUS_FULL != 0 {
      wait_for_event();
    }
    self.write(REG_WRITE, (data << 4) | (channel as u32));
    barrier();
  }

  pub fn robey3(&self) -> u32 {
    let mut prop = PropertyMailbox::new();
    prop.add(TAG_FB_SET_SIZE, &[640, 480]);
    prop.add(TAG_FB_SET_VIRTUAL_SIZE, &[640, 480]);
    prop.add(TAG_FB_SET_DEPTH, &[24]);
    prop.write(self) as u32
  }
}

impl Mmio for Mailbox {
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
    match self.buffer[1] {
      CODE_REQUEST => PropertyMailboxCode::NoReply,
      CODE_RESPONSE_OK => PropertyMailboxCode::Ok,
      CODE_RESPONSE_ERROR => PropertyMailboxCode::Error,
      _ => PropertyMailboxCode::BadReply
    }
  }

}
