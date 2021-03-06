// constants related to the raspi

// old raspi 1
// pub const PERIPHERAL_ZONE: usize = 0x2000_0000;
pub const PERIPHERAL_ZONE: usize = 0x3f00_0000;

pub const TIMER_BASE: usize = PERIPHERAL_ZONE + 0x3000;
pub const INTERRUPTS_BASE: usize = PERIPHERAL_ZONE + 0xb200;
pub const MAILBOX_BASE: usize = PERIPHERAL_ZONE + 0xb880;
pub const GPIO_BASE: usize = PERIPHERAL_ZONE + 0x20_0000;
pub const UART0_BASE: usize = PERIPHERAL_ZONE + 0x20_1000;
