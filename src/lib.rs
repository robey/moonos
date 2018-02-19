#![feature(asm)]
#![feature(lang_items)]
#![feature(core_intrinsics)]
#![no_std]

mod mmio;
mod uart;

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! { loop {} }

#[no_mangle]
pub extern fn robey() -> u8 {
  const UART0_BASE: u32 = 0x3f201000;
  let console = uart::Uart::new(UART0_BASE);
  console.init();
  console.putc(0x52);
  0x50
}

#[no_mangle]
pub extern fn kernel_main() {
  let console = uart::Uart::new(uart::RPI2_UART0);
  console.init();
  console.puts("hello raspi kernel world!\r\n");
  console.putc(0x52);
  console.putc(0x50);
  console.putc(10);

  loop {
    console.putc(console.getc());
    console.putc(10);
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
