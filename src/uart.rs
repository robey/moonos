use core::fmt;
use gpio::{GPIO, PudMode};
use mmio::Mmio;
use native;
use raspi;
use spinlock::Mutex;

pub static SERIAL0: Mutex<Uart> = Mutex::new(Uart::new(raspi::UART0_BASE));

// offsets into the memory-mapped uart base:
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

const FR_TX_FULL: u32 = (1 << 5);
const FR_RX_EMPTY: u32 = (1 << 4);

const LCRH_8BIT: u8 = 0x60;
const LCRH_FIFO: u8 = 0x10;
const LCRH_PARITY_NONE: u8 = 0;
const LCRH_PARITY_EVEN: u8 = 0x60;
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

// 3Mhz clock for computing bps
const FUARTCLK: u32 = 3_000_000;

#[derive(Clone, Copy, Debug)]
pub enum UartRate {
  B115200 = 115_200,
}

pub struct Uart {
  base: usize
}

impl Mmio<Reg> for Uart {
  fn base(&self) -> usize { self.base }
}

impl Uart {
  pub const fn new(base: usize) -> Uart {
    Uart { base }
  }

  pub fn init(&mut self, bps: UartRate) {
    self.write(Reg::CR, 0);
    GPIO.lock().configure_pins(PudMode::Off, &[ PIN_RXD0, PIN_TXD0 ]);

    // clear all pending interrupts
    self.write(Reg::ICR, 0x7ff);

    // the baud divisor is stored as a 16Q6 fixed point, with the integer
    // part in IBRD and the fraction in FBRD. the manual says the divisor is
    // calculated from `FUARTCLK / (16 * bps)`.
    let divisor_q6 = (FUARTCLK << 6) / ((bps as u32) << 4);
    self.write(Reg::IBRD, divisor_q6 >> 6);
    self.write(Reg::FBRD, divisor_q6 & 0x3f);

    self.write(Reg::LCRH, (LCRH_8BIT | LCRH_FIFO | LCRH_PARITY_NONE) as u32);
    self.write(Reg::IMSC, IMSC_OVERRUN | IMSC_BREAK | IMSC_PARITY | IMSC_FRAMING | IMSC_RX_TIMEOUT | IMSC_TX |
      IMSC_RX | IMSC_CTS);
    self.write(Reg::CR, CR_RX_ENABLE | CR_TX_ENABLE | CR_UART_ENABLE);
  }

  pub fn write_char(&mut self, c: u8) {
    while self.read(Reg::FR) & FR_TX_FULL != 0 {
      native::delay_cycles(10);
    }
    self.write(Reg::DR, c as u32)
  }

  pub fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
    for c in s.bytes() { self.write_char(c) }
    Ok(())
  }

  pub fn read_char(&mut self) -> u8 {
    while self.read(Reg::FR) & FR_RX_EMPTY != 0 {
      native::delay_cycles(10);
    }
    (self.read(Reg::DR) & 0xff) as u8
  }
}

impl fmt::Write for Uart {
  fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
    self.write_str(s)
  }
}
