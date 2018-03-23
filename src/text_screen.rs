use framebuffer::{Framebuffer};

pub struct BitmapFont {
  pub width: usize,
  pub height: usize,
  pub data: &'static [u8]
}

// built-in fonts:

use limoncello;
pub static LIMONCELLO: BitmapFont = BitmapFont {
  width: limoncello::FONT_WIDTH,
  height: limoncello::FONT_HEIGHT,
  data: &limoncello::FONT_DATA
};

/// a consumed Framebuffer that displays text
pub struct TextScreen {
  framebuffer: Framebuffer,
  font: &'static BitmapFont,
  pub rows: u32,
  pub cols: u32,
  pub cursor_x: u32,
  pub cursor_y: u32,
  pub fg_color: u32,
  pub bg_color: u32,
  x_offset: u32,
  y_offset: u32,
  px: u32,
  py: u32,
}

impl TextScreen {
  pub fn new(framebuffer: Framebuffer, font: &'static BitmapFont) -> TextScreen {
    let rows = framebuffer.height / font.height as u32;
    let cols = framebuffer.width / font.width as u32;
    let x_offset = (framebuffer.width - cols * font.width as u32) >> 1;
    let y_offset = (framebuffer.height - rows * font.height as u32) >> 1;
    TextScreen {
      framebuffer,
      font,
      rows,
      cols,
      cursor_x: 0,
      cursor_y: 0,
      fg_color: 0xffffff,
      bg_color: 0,
      x_offset,
      y_offset,
      px: x_offset,
      py: y_offset,
    }
  }

  // draw a character at the current position, without moving the cursor
  // or interpreting control codes.
  pub fn draw_char(&mut self, c: char) {
    let font_offset = (c as usize) * self.font.height;
    for i in 0..self.font.height {
      let py = self.py + i as u32;
      self.font.data.get(font_offset + i).map(|line| {
        self.framebuffer.blit_hline(self.px, py, *line as u32, self.font.width, self.fg_color, self.bg_color);
      });
    }
  }

  pub fn write_char(&mut self, c: char) {
    match c as u32 {
      10 => self.linefeed(),
      13 => self.cr(),
      _ => {
        self.draw_char(c);
        self.cursor_x += 1;
        self.px += self.font.width as u32;
        if self.cursor_x >= self.cols { self.linefeed(); }
      }
    }
  }

  pub fn write_string(&mut self, s: &str) {
    s.chars().for_each(|c| self.write_char(c));
  }

  pub fn cr(&mut self) {
    self.cursor_x = 0;
    self.px = self.x_offset;
  }

  pub fn linefeed(&mut self) {
    self.cr();
    self.cursor_y += 1;
    self.py += self.font.height as u32;
    if self.cursor_y >= self.rows {
      // scroll...
    }
  }

  pub fn clear(&mut self) {
    let width = self.framebuffer.width;
    let height = self.framebuffer.height;
    self.framebuffer.fill_box(0, 0, width, height, self.bg_color);
    self.cursor_x = 0;
    self.cursor_y = 0;
    self.px = self.x_offset;
    self.py = self.y_offset;
  }
}
