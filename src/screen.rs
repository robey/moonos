/// gpu-based framebuffer

use core::{intrinsics, slice};
use mailbox::{MAILBOX, PropertyMailbox, PropertyMailboxCode};
use native;
use spinlock::Mutex;

const TAG_FB_GET_FRAMEBUFFER: u32 = 0x00040001;
const TAG_FB_SET_SIZE: u32 = 0x00048003;
const TAG_FB_SET_VIRTUAL_SIZE: u32 = 0x00048004;
const TAG_FB_SET_DEPTH: u32 = 0x00048005;

// accessible version of the global screen
pub static SCREEN: Mutex<Screen> = Mutex::new(Screen::new());

// initialize the global screen.
pub fn screen_init(width: u32, height: u32, depth: u32) -> Result<(), PropertyMailboxCode> {
  let mut s = SCREEN.lock();
  s.set_size(width, height, depth).and_then(|()| s.get_framebuffer())
}

pub struct Screen {
  pub width: u32,
  pub height: u32,
  pub depth: u32,
  framebuffer: Option<&'static mut [u8]>,
}

impl Screen {
  pub const fn new() -> Screen {
    Screen { width: 0, height: 0, depth: 0, framebuffer: None }
  }

  fn bpp(&self) -> u32 { self.depth >> 3 }

  fn pitch(&self) -> u32 { self.width * self.bpp() }

  pub fn set_size(&mut self, width: u32, height: u32, depth: u32) -> Result<(), PropertyMailboxCode> {
    let mut prop = PropertyMailbox::new();
    prop.add(TAG_FB_SET_SIZE, &[ width, height ]);
    prop.add(TAG_FB_SET_VIRTUAL_SIZE, &[ width, height ]);
    prop.add(TAG_FB_SET_DEPTH, &[ depth ]);

    let rv = prop.write(&mut MAILBOX.lock());
    if rv != PropertyMailboxCode::Ok { return Err(rv) }

    self.width = width;
    self.height = height;
    self.depth = depth;
    Ok(())
  }

  pub fn get_framebuffer(&mut self) -> Result<(), PropertyMailboxCode> {
    let mut prop = PropertyMailbox::new();
    // request align(16)
    prop.add(TAG_FB_GET_FRAMEBUFFER, &[ 16, 0 ]);

    let rv = prop.write(&mut MAILBOX.lock());
    if rv != PropertyMailboxCode::Ok { return Err(rv) }

    if let Some(&[ address, size ]) = prop.tag_result(TAG_FB_GET_FRAMEBUFFER) {
      let buffer = address as usize as *mut u8;
      self.framebuffer = unsafe { Some(slice::from_raw_parts_mut(buffer, size as usize)) };
      Ok(())
    } else {
      Err(PropertyMailboxCode::BadReply)
    }
  }

  #[inline]
  fn put_pixel(&mut self, offset: u32, color: u32) {
    let bpp = self.bpp();
    self.framebuffer.as_mut().map(|fb| {
      for i in 0..bpp {
        let byte = ((color >> (i * 8)) & 0xff) as u8;
        unsafe {
          fb.get_mut((offset + i) as usize).map(|fb| intrinsics::volatile_store(fb, byte));
        }
      }
    });
  }

  pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
    let offset = y * self.pitch() + x * self.bpp();
    self.put_pixel(offset, color);
  }

  pub fn fill_box(&mut self, x: u32, y: u32, x2: u32, y2: u32, color: u32) {
    let pitch = self.pitch();
    let width = (x2 - x) * self.bpp();
    let mut line = (y * pitch + x * self.bpp()) as usize;

    let mut offset = line as u32;
    for _px in 0..(x2 - x) {
      self.put_pixel(offset, color);
      offset += self.bpp();
    }
    line += pitch as usize;

    self.framebuffer.as_mut().map(|fb| {
      for _py in 1..(y2 - y) {
        let source = line - pitch as usize;
        let dest = line;
        unsafe {
          native::copy_memory(&mut fb[dest] as *mut u8, &mut fb[source] as *const u8, width as usize);
        }
        line += pitch as usize;
      }
    });
  }

  pub fn scroll_up(&mut self, lines: u32) {
    let source_offset = (lines * self.pitch()) as usize;
    let dest_offset = 0;
    let count = (self.pitch() * (self.height - lines)) as usize;
    self.framebuffer.as_mut().map(|fb| {
      unsafe {
        native::copy_memory(&mut fb[dest_offset] as *mut u8, &mut fb[source_offset] as *const u8, count);
      }
    });
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
