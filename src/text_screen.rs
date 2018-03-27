use core::fmt;
use framebuffer::{Framebuffer};

const REPLACEMENT_CHAR: char = 0xfffd as char;

pub struct BitmapFont {
  pub width: usize,
  pub height: usize,
  pub data: &'static [u8],
  pub codepoints: &'static [u32],
  pub codepoints_map: &'static [usize],
}

// built-in fonts:

use limoncello;
pub static LIMONCELLO: BitmapFont = BitmapFont {
  width: limoncello::FONT_WIDTH,
  height: limoncello::FONT_HEIGHT,
  data: &limoncello::FONT_DATA,
  codepoints: &limoncello::FONT_CODEPOINTS,
  codepoints_map: &limoncello::FONT_CODEPOINTS_MAP,
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

  pub fn move_to(&mut self, x: u32, y: u32) {
    self.cursor_x = x;
    self.cursor_y = y;
    if self.cursor_x >= self.cols { self.cursor_x = 0; }
    if self.cursor_y >= self.rows { self.cursor_y = 0; }
    self.px = self.x_offset + self.cursor_x * self.font.width as u32;
    self.py = self.y_offset + self.cursor_y * self.font.height as u32;
  }

  // draw a character at the current position, without moving the cursor
  // or interpreting control codes.
  pub fn draw_char(&mut self, c: char) {
    if let Ok(index) = self.font.codepoints.binary_search(&(c as u32)).map(|i| self.font.codepoints_map[i]) {
      let font_offset = index * self.font.height;
      for i in 0..self.font.height {
        let py = self.py + i as u32;
        self.font.data.get(font_offset + i).map(|line| {
          self.framebuffer.blit_hline(self.px, py, *line as u32, self.font.width, self.fg_color, self.bg_color);
        });
      }
    } else if c != REPLACEMENT_CHAR {
      self.draw_char(REPLACEMENT_CHAR);
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
    if self.cursor_y >= self.rows - 1 {
      self.scroll_up();
    } else {
      self.cursor_y += 1;
      self.py += self.font.height as u32;
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

  pub fn clear_line(&mut self, y: u32) {
    let width = self.framebuffer.width;
    let y_top = self.y_offset + y * self.font.height as u32;
    let y_bottom = y_top + self.font.height as u32;
    self.framebuffer.fill_box(0, y_top, width, y_bottom, self.bg_color);
  }

  fn scroll_up(&mut self) {
    let width = self.framebuffer.width;
    let y_top = self.y_offset + self.font.height as u32;
    let y_bottom = self.y_offset + self.font.height as u32 * self.rows;
    self.framebuffer.move_box(0, y_top, width, y_bottom, 0, 0);
    let rows = self.rows;
    self.clear_line(rows - 1);
  }
}

impl fmt::Write for TextScreen {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);
    Ok(())
  }
}
