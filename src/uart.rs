use mmio::{gpio, Mmio, PudMode};

// offsets into the memory-mapped uart base:
// (can't use enum because rust doesn't understand that it's an isize)
const REG_DR: isize = 0x00;
const REG_FR: isize = 0x18;
const REG_IBRD: isize = 0x24;
const REG_FBRD: isize = 0x28;
const REG_LCRH: isize = 0x2c;
const REG_CR: isize = 0x30;
const REG_IMSC: isize = 0x38;
const REG_ICR: isize = 0x44;

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

/*
enum
{
    // The base address for UART.
    UART0_BASE = 0x3F201000, // for raspi2 & 3, 0x20201000 for raspi1

    UART0_DR     = (UART0_BASE + 0x00),
    UART0_RSRECR = (UART0_BASE + 0x04),
    UART0_FR     = (UART0_BASE + 0x18),
    UART0_ILPR   = (UART0_BASE + 0x20),
    UART0_IBRD   = (UART0_BASE + 0x24),
    UART0_FBRD   = (UART0_BASE + 0x28),
    UART0_LCRH   = (UART0_BASE + 0x2C),
    UART0_CR     = (UART0_BASE + 0x30),
    UART0_IFLS   = (UART0_BASE + 0x34),
    UART0_IMSC   = (UART0_BASE + 0x38),
    UART0_RIS    = (UART0_BASE + 0x3C),
    UART0_MIS    = (UART0_BASE + 0x40),
    UART0_ICR    = (UART0_BASE + 0x44),
    UART0_DMACR  = (UART0_BASE + 0x48),
    UART0_ITCR   = (UART0_BASE + 0x80),
    UART0_ITIP   = (UART0_BASE + 0x84),
    UART0_ITOP   = (UART0_BASE + 0x88),
    UART0_TDR    = (UART0_BASE + 0x8C),
};
*/



pub struct Uart {
  base: *mut u8
}

impl Mmio for Uart {
  #[inline]
  fn base(&self) -> *mut u8 { self.base }
}

impl Uart {
  pub fn new(base: u32) -> Uart {
    Uart { base: base as *mut u8 }
  }

  pub fn init(&self) {
    self.write(REG_CR, 0);
    gpio().configure_pins(PudMode::Off, &[ PIN_RXD0, PIN_TXD0 ]);

    // clear all pending interrupts
    self.write(REG_ICR, 0x7ff);

    // the baud divisor is stored as a 16Q6 fixed point, with the integer
    // part in IBRD and the fraction in FBRD. the manual says the divisor is
    // calculated from `FUARTCLK / (16 * baudrate)`.
    // ... this is 1.625, but i can't figure out what baud rate is intended.
    // a GPU clock speed of 250Mhz would be 9.6Mbps?!
    self.write(REG_IBRD, 1);
    self.write(REG_FBRD, 40);

    self.write(REG_LCRH, (LCRH_8BIT | LCRH_FIFO | LCRH_PARITY_NONE) as u32);
    self.write(REG_IMSC, IMSC_OVERRUN | IMSC_BREAK | IMSC_PARITY | IMSC_FRAMING | IMSC_RX_TIMEOUT | IMSC_TX |
      IMSC_RX | IMSC_CTS);
    self.write(REG_CR, CR_RX_ENABLE | CR_TX_ENABLE | CR_UART_ENABLE);
  }

  pub fn putc(&self, c: u8) {
    while self.read(REG_FR) & FR_TX_FULL != 0 {}
    self.write(REG_DR, c as u32)
  }

  pub fn puts(&self, s: &str) {
    for c in s.bytes() { self.putc(c) }
  }

  pub fn getc(&self) -> u8 {
    while self.read(REG_FR) & FR_RX_EMPTY != 0 {
      unsafe { asm!("wfe") }
    }
    self.putc(0x21);
    (self.read(REG_DR) & 0xff) as u8
  }
}
