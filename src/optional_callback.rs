use core::intrinsics;

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
    let value = callback.map(|c| c as *const usize as usize).unwrap_or(0);
    // this is an atomic operation, so do some ju-jitsu to cast into mutability.
    let ptr = &self.pointer as *const usize as usize as *mut usize;
    unsafe { intrinsics::atomic_store(ptr, value); }
  }

  pub fn get(&self) -> Option<Callback> {
    let ptr = &self.pointer as *const usize;
    let value_ptr = unsafe { intrinsics::volatile_load(ptr) as *const usize };
    if value_ptr.is_null() {
      None
    } else {
      // rust doesn't mind casting `Callback -> *const`, but it will be
      // damned if it will cast `*const -> Callback`! so add a layer of
      // indirection.
      Some(unsafe { *(&value_ptr as *const *const usize as *const Callback) })
    }
  }
}
