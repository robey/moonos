/// gpu-based framebuffer

use core::slice;
use mailbox::{mailbox, PropertyMailbox, PropertyMailboxCode};

const TAG_FB_GET_FRAMEBUFFER: u32 = 0x00040001;
const TAG_FB_SET_SIZE: u32 = 0x00048003;
const TAG_FB_SET_VIRTUAL_SIZE: u32 = 0x00048004;
const TAG_FB_SET_DEPTH: u32 = 0x00048005;

static mut EMPTY: [u8; 0] = [0; 0];

pub struct Framebuffer {
  width: u32,
  height: u32,
  depth: u32,
  framebuffer: &'static mut [u8]
}

impl Framebuffer {
  pub fn new() -> Framebuffer {
    unsafe {
      Framebuffer { width: 0, height: 0, depth: 0, framebuffer: &mut EMPTY }
    }
  }

  pub fn set_size(&mut self, width: u32, height: u32, depth: u32) -> PropertyMailboxCode {
    let mut prop = PropertyMailbox::new();
    prop.add(TAG_FB_SET_SIZE, &[ width, height ]);
    prop.add(TAG_FB_SET_VIRTUAL_SIZE, &[ width, height ]);
    prop.add(TAG_FB_SET_DEPTH, &[ depth ]);

    let rv = prop.write(&mailbox());
    if rv != PropertyMailboxCode::Ok { return rv }

    self.width = width;
    self.height = height;
    self.depth = depth;
    rv
  }

  pub fn get_framebuffer(&mut self) -> PropertyMailboxCode {
    let mut prop = PropertyMailbox::new();
    // request align(16)
    prop.add(TAG_FB_GET_FRAMEBUFFER, &[ 16, 0 ]);

    let rv = prop.write(&mailbox());
    if rv != PropertyMailboxCode::Ok { return rv }

    if let Some(&[ address, size ]) = prop.tag_result(TAG_FB_GET_FRAMEBUFFER) {
      let framebuffer = unsafe { slice::from_raw_parts_mut(address as usize as *mut u8, size as usize) };
      self.framebuffer = framebuffer;
      PropertyMailboxCode::Ok
    } else {
      PropertyMailboxCode::BadReply
    }
  }

  pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
    // if self.framebuffer.is_some() {
      let bpp = self.depth >> 3;
      let pitch = self.width * bpp;
      let offset = y * pitch + x * bpp;
      for i in 0..bpp {
        self.framebuffer[(offset + i) as usize] = ((color >> (i * 8)) & 0xff) as u8;
      }
    // }
  }
}

pub fn framebuffer() -> Framebuffer {
  Framebuffer::new()
}
