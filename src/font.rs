use framebuffer::{Framebuffer};

pub struct Font {
  width: usize,
  height: usize,
  data: &'static [u8]
}

impl Font {
  pub fn new(width: usize, height: usize, data: &'static [u8]) -> Font {
    Font { width, height, data }
  }

  pub fn putc(&self, fb: &mut Framebuffer, x: u32, y: u32, c: u8, fg: u32, bg: u32) {
    let start = (c as usize) * self.height;
    for py in 0..self.height {
      fb.blit_hline(x, y + py as u32, self.data[start + py] as u32, self.width, fg, bg);
    }
  }
}

pub struct Console {
  framebuffer: &'static Framebuffer,
  // font: &'static Font,
}

impl Console {
  pub fn new(framebuffer: &'static Framebuffer) -> Console {
    Console { framebuffer, }
  }
}
