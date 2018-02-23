/// gpu-based framebuffer

use core::slice;
use mailbox::{mailbox, PropertyMailbox, PropertyMailboxCode};

const TAG_FB_GET_FRAMEBUFFER: u32 = 0x00040001;
const TAG_FB_SET_SIZE: u32 = 0x00048003;
const TAG_FB_SET_VIRTUAL_SIZE: u32 = 0x00048004;
const TAG_FB_SET_DEPTH: u32 = 0x00048005;

pub struct Framebuffer {
  width: u32,
  height: u32,
  depth: u32,
  framebuffer: Option<&'static mut [u8]>,
}

impl Framebuffer {
  pub fn new() -> Framebuffer {
    Framebuffer { width: 0, height: 0, depth: 0, framebuffer: None }
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
      self.framebuffer = unsafe { Some(slice::from_raw_parts_mut(address as usize as *mut u8, size as usize)) };
      PropertyMailboxCode::Ok
    } else {
      PropertyMailboxCode::BadReply
    }
  }

  #[inline]
  fn put_pixel(&mut self, offset: u32, color: u32) {
    let bpp = self.depth >> 3;
    self.framebuffer.as_mut().map(|fb| {
      for i in 0..bpp {
        fb.get_mut((offset + i) as usize).map(|fb| *fb = ((color >> (i * 8)) & 0xff) as u8);
      }
    });
  }

  pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
    if self.framebuffer.is_none() { return }
    let bpp = self.depth >> 3;
    let pitch = self.width * bpp;
    let offset = y * pitch + x * bpp;
    self.put_pixel(offset, color);
  }

  pub fn blit_glyph(&mut self, x: u32, y: u32, height: usize, glyph: &[u32], fg: u32, bg: u32) {
    if self.framebuffer.is_none() { return }
    let bpp = self.depth >> 3;
    let pitch = self.width * bpp;
    let mut line = y * pitch + x * bpp;
    let mut offset = line;
    for py in 0..height {
      for px in 0..glyph.len() {
        let color = if (glyph[px] >> py) & 1 != 0 { fg } else { bg };
        self.put_pixel(offset, color);
        offset += bpp;
      }
      line += pitch;
      offset = line;
    }
  }
}

pub fn framebuffer() -> Framebuffer {
  Framebuffer::new()
}
