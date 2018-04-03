// interrupt vectors

.section ".text.vectors"

.global _exception_vector
.global _saved_registers

.macro save_all_registers
//  stmfd sp!, {r0-r15}^
  stmfd sp!, {r0-r15}
  mrs r12, apsr
  stmfd sp!, {r12}
.endm

.macro unsave_all_registers
  add sp, sp, #68
.endm

_exception_vector:
  ldr pc, vector_reset_addr
  ldr pc, vector_undefined_addr
  ldr pc, vector_syscall_addr
  nop
  nop
  nop
  nop
  nop

//  ldr pc, prefetch_abort_handler_abs_addr
//  ldr pc, data_abort_handler_abs_addr
//  nop
//  ldr pc, irq_handler_abs_addr
//  ldr pc, fast_irq_handler_abs_addr

vector_reset_addr:          .word vector_reset
vector_undefined_addr:      .word vector_undefined_setup
vector_syscall_addr:        .word vector_syscall_setup
@ prefetch_abort_handler_abs_addr:        .word prefetch_abort_handler
@ data_abort_handler_abs_addr:            .word data_abort_handler
@ irq_handler_abs_addr:                   .word irq_handler_asm_wrapper
@ fast_irq_handler_abs_addr:              .word fast_irq_handler

vector_undefined_setup:
  save_all_registers
  stmfd sp!, {lr}
  blx vector_undefined
  ldmia sp!, {lr}
  unsave_all_registers
  bx lr

// `SVC #<u24>`
// arm convention is to get the SVC number by reading the source instruction
// (which is in `[lr, #-4]`). however, this pollutes the data cache with
// an instruction block for no reason, so modern OS use a register for this.
// we choose r4, pikachu!
vector_syscall_setup:
  save_all_registers
  stmfd sp!, {r5, r6, lr}
  add r3, sp, #12
  ldr r6, =vector_syscalls
  ldr r5, =vector_syscalls_end
  ldr r6, [r6, r4, lsl #2]
  cmp r5, r6
  ldrge r6, =syscall_zero
  blx r6
  ldmia sp!, {r5, r6, lr}
  unsave_all_registers
  bx lr

vector_syscalls:
  .word syscall_zero
  .word syscall_one
vector_syscalls_end:

// .data
// .section ".data.saved_registers"
//
// _saved_registers:
//   .rept 14
//   .word 0x23
//   .endr
//   .word 0x19
//   .word 0xff003322
