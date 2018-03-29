// row data, little-endian (smallest bit on left)

#[allow(dead_code)]
pub const FONT_GLYPHS: usize = 256;
#[allow(dead_code)]
pub const FONT_HEIGHT: usize = 14;
#[allow(dead_code)]
pub const FONT_WIDTH: usize = 8;

#[allow(dead_code)]
pub const FONT_DATA: [u8; 3584] = [
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, // 5
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, // 10
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, // 15
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, // 20
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x00, 0x07, 0x05, 0x57, 0x25, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, // 25
  0x00, 0x00, 0x00, 0x00, 0x15, 0x15, 0x1d, 0x11, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, // 30
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x12, 0x12, 0x12, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x12, 0x12, 0x3f, 0x12, 0x12, 0x12, 0x3f, 0x12, 0x12, 0x00, 0x00, 0x00, 0x00, // 35
  0x08, 0x08, 0x3c, 0x0a, 0x0a, 0x1c, 0x28, 0x28, 0x1e, 0x08, 0x08, 0x00, 0x00, 0x00, 
  0x00, 0x02, 0x05, 0x22, 0x10, 0x08, 0x04, 0x22, 0x50, 0x20, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x04, 0x0a, 0x0a, 0x04, 0x4a, 0x29, 0x11, 0x11, 0x6e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x04, 0x04, 0x02, 0x02, 0x02, 0x02, 0x02, 0x04, 0x04, 0x08, 0x00, 0x00, 0x00, // 40
  0x04, 0x08, 0x08, 0x10, 0x10, 0x10, 0x10, 0x10, 0x08, 0x08, 0x04, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x08, 0x2a, 0x1c, 0x08, 0x1c, 0x2a, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x08, 0x08, 0x08, 0x7f, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x08, 0x04, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 45
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 
  0x10, 0x10, 0x08, 0x08, 0x04, 0x04, 0x02, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x1e, 0x21, 0x21, 0x21, 0x2d, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x08, 0x0c, 0x0a, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x1e, 0x21, 0x20, 0x20, 0x18, 0x04, 0x02, 0x01, 0x3f, 0x00, 0x00, 0x00, 0x00, // 50
  0x00, 0x3f, 0x20, 0x10, 0x08, 0x1c, 0x20, 0x20, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x10, 0x18, 0x14, 0x12, 0x11, 0x3f, 0x10, 0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x3f, 0x01, 0x01, 0x1f, 0x20, 0x20, 0x20, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x1c, 0x02, 0x01, 0x01, 0x1f, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x3f, 0x20, 0x10, 0x10, 0x08, 0x08, 0x04, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, // 55
  0x00, 0x1e, 0x21, 0x21, 0x21, 0x1e, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x1e, 0x21, 0x21, 0x21, 0x3e, 0x20, 0x20, 0x10, 0x0e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x04, 0x04, 0x00, 0x00, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x04, 0x04, 0x00, 0x00, 0x04, 0x04, 0x04, 0x02, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x10, 0x08, 0x04, 0x02, 0x04, 0x08, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, // 60
  0x00, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x02, 0x04, 0x08, 0x10, 0x08, 0x04, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x1e, 0x21, 0x20, 0x10, 0x08, 0x04, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x0c, 0x12, 0x21, 0x2d, 0x2d, 0x25, 0x19, 0x02, 0x3c, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x08, 0x08, 0x14, 0x14, 0x22, 0x3e, 0x22, 0x41, 0x41, 0x00, 0x00, 0x00, 0x00, // 65
  0x00, 0x1f, 0x22, 0x22, 0x22, 0x1e, 0x22, 0x22, 0x22, 0x1f, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x1c, 0x22, 0x01, 0x01, 0x01, 0x01, 0x01, 0x22, 0x1c, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x0f, 0x12, 0x12, 0x22, 0x22, 0x22, 0x12, 0x12, 0x0f, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x3f, 0x01, 0x01, 0x01, 0x0f, 0x01, 0x01, 0x01, 0x3f, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x3f, 0x01, 0x01, 0x01, 0x1f, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, // 70
  0x00, 0x3c, 0x02, 0x01, 0x01, 0x39, 0x21, 0x21, 0x22, 0x1c, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x21, 0x21, 0x21, 0x21, 0x3f, 0x21, 0x21, 0x21, 0x21, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x21, 0x11, 0x09, 0x05, 0x03, 0x05, 0x09, 0x11, 0x21, 0x00, 0x00, 0x00, 0x00, // 75
  0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x3f, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x41, 0x63, 0x63, 0x55, 0x55, 0x49, 0x49, 0x41, 0x41, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x21, 0x23, 0x25, 0x25, 0x29, 0x29, 0x31, 0x21, 0x21, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x1f, 0x21, 0x21, 0x21, 0x1f, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, // 80
  0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x29, 0x29, 0x11, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x1f, 0x21, 0x21, 0x21, 0x1f, 0x11, 0x11, 0x21, 0x21, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x1e, 0x21, 0x01, 0x01, 0x1e, 0x20, 0x20, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x7f, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, // 85
  0x00, 0x41, 0x41, 0x41, 0x22, 0x22, 0x14, 0x14, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x41, 0x41, 0x49, 0x49, 0x49, 0x55, 0x55, 0x22, 0x22, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x21, 0x21, 0x12, 0x12, 0x0c, 0x12, 0x12, 0x21, 0x21, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x41, 0x41, 0x22, 0x14, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x3f, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01, 0x01, 0x3f, 0x00, 0x00, 0x00, 0x00, // 90
  0x1e, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x1e, 0x00, 0x00, 0x00, 
  0x02, 0x02, 0x04, 0x04, 0x08, 0x08, 0x10, 0x10, 0x20, 0x20, 0x00, 0x00, 0x00, 0x00, 
  0x1e, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1e, 0x00, 0x00, 0x00, 
  0x00, 0x08, 0x14, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x00, // 95
  0x00, 0x04, 0x04, 0x04, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x1e, 0x20, 0x38, 0x26, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x01, 0x01, 0x01, 0x0d, 0x13, 0x21, 0x21, 0x21, 0x11, 0x0f, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x1c, 0x22, 0x01, 0x01, 0x01, 0x22, 0x1c, 0x00, 0x00, 0x00, 0x00, 
  0x20, 0x20, 0x20, 0x2c, 0x32, 0x21, 0x21, 0x21, 0x22, 0x3c, 0x00, 0x00, 0x00, 0x00, // 100
  0x00, 0x00, 0x00, 0x1e, 0x21, 0x21, 0x3f, 0x01, 0x01, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x38, 0x44, 0x04, 0x04, 0x3f, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x2c, 0x32, 0x21, 0x21, 0x21, 0x22, 0x3c, 0x20, 0x21, 0x1e, 0x00, 
  0x01, 0x01, 0x01, 0x1d, 0x23, 0x21, 0x21, 0x21, 0x21, 0x21, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x00, 0x00, 0x0e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00, 0x00, // 105
  0x10, 0x00, 0x00, 0x1c, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x11, 0x0e, 0x00, 0x00, 
  0x01, 0x01, 0x01, 0x21, 0x11, 0x09, 0x05, 0x0b, 0x11, 0x21, 0x00, 0x00, 0x00, 0x00, 
  0x0e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x37, 0x49, 0x49, 0x49, 0x49, 0x49, 0x49, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x1d, 0x23, 0x21, 0x21, 0x21, 0x21, 0x21, 0x00, 0x00, 0x00, 0x00, // 110
  0x00, 0x00, 0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x0d, 0x13, 0x21, 0x21, 0x21, 0x11, 0x0f, 0x01, 0x01, 0x01, 0x00, 
  0x00, 0x00, 0x00, 0x2c, 0x32, 0x21, 0x21, 0x21, 0x22, 0x3c, 0x20, 0x20, 0x60, 0x00, 
  0x00, 0x00, 0x00, 0x1d, 0x23, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x1e, 0x21, 0x01, 0x1e, 0x20, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, // 115
  0x04, 0x04, 0x04, 0x1f, 0x04, 0x04, 0x04, 0x04, 0x04, 0x38, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x41, 0x41, 0x22, 0x22, 0x14, 0x14, 0x08, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x41, 0x41, 0x49, 0x49, 0x55, 0x22, 0x22, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x21, 0x21, 0x12, 0x0c, 0x12, 0x21, 0x21, 0x00, 0x00, 0x00, 0x00, // 120
  0x00, 0x00, 0x00, 0x21, 0x21, 0x21, 0x12, 0x12, 0x12, 0x0c, 0x08, 0x04, 0x03, 0x00, 
  0x00, 0x00, 0x00, 0x3f, 0x20, 0x10, 0x0c, 0x02, 0x01, 0x3f, 0x00, 0x00, 0x00, 0x00, 
  0x30, 0x08, 0x08, 0x08, 0x04, 0x03, 0x04, 0x08, 0x08, 0x08, 0x30, 0x00, 0x00, 0x00, 
  0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 
  0x03, 0x04, 0x04, 0x04, 0x08, 0x30, 0x08, 0x04, 0x04, 0x04, 0x03, 0x00, 0x00, 0x00, // 125
  0x00, 0x00, 0x00, 0x00, 0x00, 0x26, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x1c, 0x1c, 0x36, 0x2a, 0x6f, 0x36, 0x3e, 0x14, 0x1c, 0x08, 0x00, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x08, 0x14, 0x00, 0x1e, 0x20, 0x38, 0x26, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x04, 0x00, 0x1e, 0x20, 0x38, 0x26, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, // 130
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, // 135
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, // 140
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, // 145
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, // 150
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, // 155
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x00, 0x00, 0x00, 0x00, // 160
  0x00, 0x04, 0x00, 0x00, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x08, 0x1c, 0x2a, 0x09, 0x09, 0x09, 0x2a, 0x1c, 0x08, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x08, 0x14, 0x04, 0x0e, 0x04, 0x04, 0x06, 0x25, 0x1a, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x1c, 0x22, 0x02, 0x0f, 0x02, 0x0f, 0x02, 0x22, 0x1c, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x41, 0x22, 0x14, 0x08, 0x3e, 0x08, 0x3e, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, // 165
  0x00, 0x3f, 0x08, 0x3f, 0x08, 0x07, 0x02, 0x04, 0x08, 0x10, 0x00, 0x00, 0x00, 0x00, 
  0x1c, 0x22, 0x41, 0x59, 0x45, 0x45, 0x45, 0x59, 0x41, 0x22, 0x1c, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x00, 0x0c, 0x1e, 0x1e, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x22, 0x14, 0x08, 0x14, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 170
  0x00, 0x00, 0x08, 0x08, 0x3e, 0x08, 0x08, 0x00, 0x3e, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x00, 0x00, 0x08, 0x00, 0x00, 0x7f, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, // 175
  0x0c, 0x12, 0x12, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x0e, 0x10, 0x08, 0x04, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x0e, 0x10, 0x0c, 0x10, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, // 180
  0x00, 0x00, 0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x23, 0x5d, 0x01, 0x01, 0x01, 0x00, 
  0x00, 0x00, 0x00, 0x3f, 0x12, 0x12, 0x12, 0x12, 0x12, 0x22, 0x00, 0x00, 0x00, 0x00, 
  0x1c, 0x22, 0x41, 0x4d, 0x55, 0x4d, 0x55, 0x55, 0x41, 0x22, 0x1c, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, // 185
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x24, 0x12, 0x09, 0x12, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x09, 0x12, 0x24, 0x12, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 190
  0x00, 0x08, 0x00, 0x00, 0x08, 0x04, 0x02, 0x01, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x04, 0x08, 0x00, 0x08, 0x14, 0x14, 0x22, 0x3e, 0x41, 0x41, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x04, 0x00, 0x08, 0x14, 0x14, 0x22, 0x3e, 0x41, 0x41, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x14, 0x00, 0x08, 0x14, 0x14, 0x22, 0x3e, 0x41, 0x41, 0x00, 0x00, 0x00, 0x00, 
  0x2c, 0x1a, 0x00, 0x08, 0x14, 0x14, 0x22, 0x3e, 0x41, 0x41, 0x00, 0x00, 0x00, 0x00, // 195
  0x00, 0x22, 0x00, 0x08, 0x14, 0x14, 0x22, 0x3e, 0x41, 0x41, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x08, 0x14, 0x08, 0x14, 0x14, 0x22, 0x3e, 0x41, 0x41, 0x00, 0x00, 0x00, 0x00, 
  0x04, 0x08, 0x00, 0x3f, 0x01, 0x01, 0x0f, 0x01, 0x01, 0x3f, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x04, 0x00, 0x3f, 0x01, 0x01, 0x0f, 0x01, 0x01, 0x3f, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x14, 0x00, 0x3f, 0x01, 0x01, 0x0f, 0x01, 0x01, 0x3f, 0x00, 0x00, 0x00, 0x00, // 200
  0x00, 0x12, 0x00, 0x3f, 0x01, 0x01, 0x0f, 0x01, 0x01, 0x3f, 0x00, 0x00, 0x00, 0x00, 
  0x04, 0x08, 0x00, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x10, 0x08, 0x00, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x14, 0x00, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x14, 0x00, 0x3e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00, 0x00, // 205
  0x00, 0x1c, 0x22, 0x01, 0x01, 0x01, 0x01, 0x01, 0x22, 0x1c, 0x08, 0x08, 0x06, 0x00, 
  0x26, 0x19, 0x00, 0x21, 0x23, 0x25, 0x29, 0x31, 0x21, 0x21, 0x00, 0x00, 0x00, 0x00, 
  0x04, 0x08, 0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x04, 0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x14, 0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, // 210
  0x14, 0x0a, 0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x12, 0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x1e, 0x21, 0x31, 0x29, 0x25, 0x23, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x04, 0x08, 0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x04, 0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, // 215
  0x08, 0x14, 0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x14, 0x0a, 0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x12, 0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x10, 0x08, 0x00, 0x41, 0x22, 0x14, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x14, 0x00, 0x41, 0x22, 0x14, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, // 220
  0x00, 0x0f, 0x12, 0x12, 0x22, 0x27, 0x22, 0x12, 0x12, 0x0f, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x07, 0x02, 0x1e, 0x22, 0x22, 0x22, 0x1e, 0x02, 0x07, 0x00, 0x00, 0x00, 0x00, 
  0x21, 0x7f, 0x21, 0x25, 0x29, 0x25, 0x29, 0x25, 0x21, 0x7f, 0x21, 0x21, 0x00, 0x00, 
  0x04, 0x08, 0x00, 0x1e, 0x20, 0x38, 0x26, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x04, 0x00, 0x1e, 0x20, 0x38, 0x26, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, // 225
  0x08, 0x14, 0x00, 0x1e, 0x20, 0x38, 0x26, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x14, 0x0a, 0x00, 0x1e, 0x20, 0x38, 0x26, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x12, 0x00, 0x1e, 0x20, 0x38, 0x26, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x14, 0x08, 0x1e, 0x20, 0x38, 0x26, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x04, 0x08, 0x00, 0x1e, 0x21, 0x21, 0x3f, 0x01, 0x01, 0x3e, 0x00, 0x00, 0x00, 0x00, // 230
  0x08, 0x04, 0x00, 0x1e, 0x21, 0x21, 0x3f, 0x01, 0x01, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x14, 0x00, 0x1e, 0x21, 0x21, 0x3f, 0x01, 0x01, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x12, 0x00, 0x1e, 0x21, 0x21, 0x3f, 0x01, 0x01, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x04, 0x08, 0x00, 0x0e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x04, 0x00, 0x0e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00, 0x00, // 235
  0x08, 0x14, 0x00, 0x0e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x14, 0x00, 0x0e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x1c, 0x22, 0x01, 0x01, 0x01, 0x22, 0x1c, 0x08, 0x08, 0x06, 0x00, 
  0x26, 0x19, 0x00, 0x1d, 0x23, 0x21, 0x21, 0x21, 0x21, 0x21, 0x00, 0x00, 0x00, 0x00, 
  0x04, 0x08, 0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, // 240
  0x08, 0x04, 0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x14, 0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x14, 0x0a, 0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x12, 0x00, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x00, 0x00, 0x1e, 0x31, 0x29, 0x25, 0x23, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, // 245
  0x04, 0x08, 0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x04, 0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x08, 0x14, 0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x14, 0x0a, 0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, 
  0x00, 0x12, 0x00, 0x21, 0x21, 0x21, 0x21, 0x21, 0x31, 0x2e, 0x00, 0x00, 0x00, 0x00, // 250
  0x08, 0x04, 0x00, 0x21, 0x21, 0x21, 0x12, 0x12, 0x12, 0x0c, 0x08, 0x04, 0x03, 0x00, 
  0x00, 0x12, 0x00, 0x21, 0x21, 0x21, 0x12, 0x12, 0x12, 0x0c, 0x08, 0x04, 0x03, 0x00, 
  0x04, 0x18, 0x0c, 0x10, 0x1e, 0x21, 0x21, 0x21, 0x21, 0x1e, 0x00, 0x00, 0x00, 0x00, 
  0x01, 0x01, 0x01, 0x1d, 0x23, 0x21, 0x21, 0x21, 0x23, 0x1d, 0x01, 0x01, 0x01, 0x00, 
  0x00, 0x0e, 0x11, 0x11, 0x0d, 0x11, 0x21, 0x21, 0x21, 0x1d, 0x00, 0x00, 0x00, 0x00, // 255
];

// use binary_search on FONT_CODEPOINTS to see if a codepoint is represented.
// if it is, use that index into FONT_CODEPOINTS_MAP to find the glyph index.

#[allow(dead_code)]
pub const FONT_CODEPOINTS: [u32; 256] = [ 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 161, 162, 163, 165, 169, 171, 174, 176, 178, 179, 181, 183, 187, 191, 192, 193, 193, 194, 195, 196, 197, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 360, 361, 376, 8226, 8230, 8364, 8377, 65280, 65281, 65282, 65283, 65284, 65285, 65286, 65287, 65288, 65289, 65290, 65291, 65292, 65293, 65294, 65295, 65296, 65297, 65298, 65299, 65300, 65301, 65302, 65303, 65304, 65305, 65306, 65307, 65308, 65309, 65310, 65311, 65407, 65408, 65409, 65410, 65411, 65412, 65413, 65414, 65415, 65416, 65417, 65418, 65419, 65420, 65421, 65422, 65423, 65424, 65425, 65426, 65427, 65428, 65429, 65430, 65431, 65432, 65433, 65434, 65435, 65436, 65437, 65438, 65439, 65449, 65452, 65453, 65454, 65457, 65460, 65462, 65465, 65466, 65467, 65469, 65503 ];

#[allow(dead_code)]
pub const FONT_CODEPOINTS_MAP: [usize; 256] = [ 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 161, 162, 163, 165, 167, 188, 183, 176, 178, 179, 181, 184, 190, 191, 192, 193, 171, 194, 195, 196, 197, 206, 198, 199, 200, 201, 202, 203, 204, 205, 221, 207, 208, 209, 210, 211, 212, 170, 213, 214, 215, 216, 218, 219, 222, 255, 224, 225, 226, 227, 228, 229, 238, 230, 231, 232, 233, 234, 235, 236, 237, 253, 239, 240, 241, 242, 243, 244, 175, 245, 246, 247, 248, 250, 251, 254, 252, 217, 249, 220, 168, 160, 164, 166, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 169, 172, 173, 174, 177, 180, 182, 185, 186, 187, 189, 223 ];
