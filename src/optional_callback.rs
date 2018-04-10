use core::intrinsics;
use core::mem;

pub type Callback = fn (irq: usize) -> ();

///
/// This is terrible.
///
/// It's basically impossible to create a static array of atomic pointers,
/// but the interrupt handler needs this to track which IRQs have a handler
/// set.
///
/// I take advantage of knowing that a `Callback` is really a static pointer
/// into codespace, and that 0 is (as usual) an invalid pointer. Then I do
/// things with usize-to-pointer conversion that will definitely prevent my
/// soul from ascending when I die.
///
#[derive(Clone, Copy, Debug)]
pub struct OptionalCallback {
  pointer: usize,
}

impl OptionalCallback {
  pub const fn new() -> OptionalCallback {
    OptionalCallback { pointer: 0 }
  }

  pub fn set(&self, callback: Option<Callback>) {
    let value: usize = callback.map(|c| unsafe { mem::transmute(c) }).unwrap_or(0);
    // this is an atomic operation, so do some ju-jitsu to cast into mutability.
    let ptr: *mut usize = unsafe { mem::transmute(&self.pointer) };
    unsafe { intrinsics::atomic_store(ptr, value); }
  }

  pub fn get(&self) -> Option<Callback> {
    let ptr: *const usize = unsafe { mem::transmute(&self.pointer) };
    let value_ptr = unsafe { intrinsics::volatile_load(ptr) as *const usize };
    if value_ptr.is_null() {
      None
    } else {
      Some(unsafe { mem::transmute(value_ptr) })
    }
  }
}
