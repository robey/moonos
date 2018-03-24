use gpio::{gpio, PudMode};
use mmio::Mmio;
use native;

// offsets into the memory-mapped uart base:
#[allow(dead_code)]
enum Reg {
  DR = 0x00,
  RDRECR = 0x04,
  FR = 0x18,
  ILPR = 0x20,
  IBRD = 0x24,
  FBRD = 0x28,
  LCRH = 0x2c,
  CR = 0x30,
  IFLS = 0x34,
  IMSC = 0x38,
  RIS = 0x3c,
  MIS = 0x40,
  ICR = 0x44,
  DMACR = 0x48,
}

impl Into<isize> for Reg {
  fn into(self) -> isize { self as isize }
}

// pin assignments
const PIN_TXD0: usize = 14;
const PIN_RXD0: usize = 15;

#[allow(dead_code)]
pub const RPI1_UART0: u32 = 0x20201000;
pub const RPI2_UART0: u32 = 0x3f201000;

const FR_TX_FULL: u32 = (1 << 5);
const FR_RX_EMPTY: u32 = (1 << 4);

const LCRH_8BIT: u8 = 0x60;
const LCRH_FIFO: u8 = 0x10;
const LCRH_PARITY_NONE: u8 = 0;
#[allow(dead_code)]
const LCRH_PARITY_EVEN: u8 = 0x60;
#[allow(dead_code)]
const LCRH_PARITY_ODD: u8 = 0x20;

const CR_RX_ENABLE: u32 = (1 << 9);
const CR_TX_ENABLE: u32 = (1 << 8);
const CR_UART_ENABLE: u32 = (1 << 0);

const IMSC_OVERRUN: u32 = (1 << 10);
const IMSC_BREAK: u32 = (1 << 9);
const IMSC_PARITY: u32 = (1 << 8);
const IMSC_FRAMING: u32 = (1 << 7);
const IMSC_RX_TIMEOUT: u32 = (1 << 6);
const IMSC_TX: u32 = (1 << 5);
const IMSC_RX: u32 = (1 << 4);
const IMSC_CTS: u32 = (1 << 1);


pub struct Uart {
  base: *mut u8
}

impl Mmio<Reg> for Uart {
  #[inline]
  fn base(&self) -> *mut u8 { self.base }
}

impl Uart {
  pub fn new(base: u32) -> Uart {
    Uart { base: base as *mut u8 }
  }

  pub fn init(&self) {
    self.write(Reg::CR, 0);
    gpio().configure_pins(PudMode::Off, &[ PIN_RXD0, PIN_TXD0 ]);

    // clear all pending interrupts
    self.write(Reg::ICR, 0x7ff);

    // the baud divisor is stored as a 16Q6 fixed point, with the integer
    // part in IBRD and the fraction in FBRD. the manual says the divisor is
    // calculated from `FUARTCLK / (16 * baudrate)`.
    // ... this is 1.625, but i can't figure out what baud rate is intended.
    // a GPU clock speed of 250Mhz would be 9.6Mbps?!
    // ... if uart speed is 3Mhz, `3M / (16 * 115200)` is approx 1.6276. we
    // only get 0.016 accuracy with 16Q6.
    self.write(Reg::IBRD, 1);
    self.write(Reg::FBRD, 40);

    self.write(Reg::LCRH, (LCRH_8BIT | LCRH_FIFO | LCRH_PARITY_NONE) as u32);
    self.write(Reg::IMSC, IMSC_OVERRUN | IMSC_BREAK | IMSC_PARITY | IMSC_FRAMING | IMSC_RX_TIMEOUT | IMSC_TX |
      IMSC_RX | IMSC_CTS);
    self.write(Reg::CR, CR_RX_ENABLE | CR_TX_ENABLE | CR_UART_ENABLE);
  }

  pub fn putc(&self, c: u8) {
    while self.read(Reg::FR) & FR_TX_FULL != 0 {}
    self.write(Reg::DR, c as u32)
  }

  pub fn puts(&self, s: &str) {
    for c in s.bytes() { self.putc(c) }
  }

  pub fn put_u32(&self, n: u32) {
    self.putc(b'$');
    for i in 0..8 {
      let nybble = (n >> (28 - i * 4)) & 0xf;
      self.putc(if nybble > 9 { 0x61 + nybble - 10 } else { 0x30 + nybble } as u8);
    }
  }

  pub fn getc(&self) -> u8 {
    while self.read(Reg::FR) & FR_RX_EMPTY != 0 {
      native::delay_cycles(10);
    }
    self.putc(0x21);
    (self.read(Reg::DR) & 0xff) as u8
  }
}
