use core::fmt;
use limoncello;
use screen::{SCREEN, Screen};
use spin::Mutex;

pub static TEXT_DISPLAY: Mutex<TextDisplay> = Mutex::new(TextDisplay::new(&SCREEN, &LIMONCELLO));

const REPLACEMENT_CHAR: char = '\u{FFFD}';

pub struct BitmapFont {
  pub width: usize,
  pub height: usize,
  pub data: &'static [u8],
  pub codepoints: &'static [u32],
  pub codepoints_map: &'static [usize],
}

// built-in fonts:
pub static LIMONCELLO: BitmapFont = BitmapFont {
  width: limoncello::FONT_WIDTH,
  height: limoncello::FONT_HEIGHT,
  data: &limoncello::FONT_DATA,
  codepoints: &limoncello::FONT_CODEPOINTS,
  codepoints_map: &limoncello::FONT_CODEPOINTS_MAP,
};

/// a text display for a Screen
pub struct TextDisplay {
  screen: &'static Mutex<Screen>,
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

impl TextDisplay {
  pub const fn new(screen: &'static Mutex<Screen>, font: &'static BitmapFont) -> TextDisplay {
    // can't determine offsets or rows/cols until the screen is create and sized.
    TextDisplay {
      screen,
      font,
      rows: 24,
      cols: 80,
      cursor_x: 0,
      cursor_y: 0,
      fg_color: 0xffffff,
      bg_color: 0,
      x_offset: 0,
      y_offset: 0,
      px: 0,
      py: 0,
    }
  }

  pub fn resize(&mut self) {
    let s = self.screen.lock();
    self.rows = s.height / self.font.height as u32;
    self.cols = s.width / self.font.width as u32;
    self.x_offset = (s.width - self.cols * self.font.width as u32) >> 1;
    self.y_offset = (s.height - self.rows * self.font.height as u32) >> 1;
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
      let mut s = self.screen.lock();
      for i in 0..self.font.height {
        let py = self.py + i as u32;
        self.font.data.get(font_offset + i).map(|line| {
          s.blit_hline(self.px, py, *line as u32, self.font.width, self.fg_color, self.bg_color);
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
    self.resize();
    let mut s = self.screen.lock();
    let width = s.width;
    let height = s.height;
    s.fill_box(0, 0, width, height, self.bg_color);
    self.cursor_x = 0;
    self.cursor_y = 0;
    self.px = self.x_offset;
    self.py = self.y_offset;
  }

  pub fn clear_line(&mut self, y: u32) {
    let mut s = self.screen.lock();
    let width = s.width;
    let y_top = self.y_offset + y * self.font.height as u32;
    let y_bottom = y_top + self.font.height as u32;
    s.fill_box(0, y_top, width, y_bottom, self.bg_color);
  }

  fn scroll_up(&mut self) {
    self.screen.lock().scroll_up(self.font.height as u32);
    let rows = self.rows;
    self.clear_line(rows - 1);
  }
}

impl fmt::Write for TextDisplay {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);
    Ok(())
  }
}
