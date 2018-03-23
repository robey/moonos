/// gpu-based framebuffer

use core::slice;
use mailbox::{mailbox, PropertyMailbox, PropertyMailboxCode};

const TAG_FB_GET_FRAMEBUFFER: u32 = 0x00040001;
const TAG_FB_SET_SIZE: u32 = 0x00048003;
const TAG_FB_SET_VIRTUAL_SIZE: u32 = 0x00048004;
const TAG_FB_SET_DEPTH: u32 = 0x00048005;

pub struct Framebuffer {
  pub width: u32,
  pub height: u32,
  pub depth: u32,
  framebuffer: Option<&'static mut [u8]>,
}

impl Framebuffer {
  pub fn new() -> Framebuffer {
    Framebuffer { width: 0, height: 0, depth: 0, framebuffer: None }
  }

  fn bpp(&self) -> u32 { self.depth >> 3 }

  fn pitch(&self) -> u32 { self.width * self.bpp() }

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
      self.framebuffer = unsafe { Some(slice::from_raw_parts_mut(address as usize as *mut u8, size as usize)) };
      PropertyMailboxCode::Ok
    } else {
      PropertyMailboxCode::BadReply
    }
  }

  #[inline]
  fn put_pixel(&mut self, offset: u32, color: u32) {
    let bpp = self.bpp();
    self.framebuffer.as_mut().map(|fb| {
      for i in 0..bpp {
        fb.get_mut((offset + i) as usize).map(|fb| *fb = ((color >> (i * 8)) & 0xff) as u8);
      }
    });
  }

  pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
    let offset = y * self.pitch() + x * self.bpp();
    self.put_pixel(offset, color);
  }

  pub fn fill_box(&mut self, x: u32, y: u32, x2: u32, y2: u32, color: u32) {
    let mut line = y * self.pitch() + x * self.bpp();
    let mut offset = line;
    for _py in 0..(y2 - y) {
      for _px in 0..(x2 - x) {
        self.put_pixel(offset, color);
        offset += self.bpp();
      }
      line += self.pitch();
      offset = line;
    }
  }

  pub fn blit_hline(&mut self, x: u32, y: u32, data: u32, width: usize, fg: u32, bg: u32) {
    let mut offset = y * self.pitch() + x * self.bpp();
    let mut bits = data;
    for _px in 0..width {
      self.put_pixel(offset, if bits & 1 != 0 { fg } else { bg });
      bits >>= 1;
      offset += self.bpp();
    }
  }
}

pub fn framebuffer() -> Framebuffer {
  Framebuffer::new()
}
