// interrupt vectors

.section ".text.vectors"

.global _exception_vector

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
vector_undefined_addr:      .word vector_undefined
vector_syscall_addr:        .word vector_syscall_setup
@ prefetch_abort_handler_abs_addr:        .word prefetch_abort_handler
@ data_abort_handler_abs_addr:            .word data_abort_handler
@ irq_handler_abs_addr:                   .word irq_handler_asm_wrapper
@ fast_irq_handler_abs_addr:              .word fast_irq_handler

// `SWI #<u24>`
// arm convention is to get the SWI number by reading the source instruction
// (which is in `[r14, #-4]`). however, this pollutes the data cache with
// an instruction block for no reason, so modern OS use a register for this.
// we choose r4, pikachu!
vector_syscall_setup:
  stmfd sp!, {r5, r6, lr}
  ldr r6, =vector_syscalls
  ldr r5, =vector_syscalls_end
  ldr r6, [r6, r4, lsl #2]
  cmp r5, r6
  ldrge r6, =syscall_zero
  blx r6
  @ STMFD   sp!, {r12}
  @ LDR     r12, [r14, #-4]       ; load the SWI instruction
  @ BIC     r12, r12, #&FF000000  ; keep only the SWI number
  @ CMP     r12, #SWIcount        ; SWI in range?
  @ LDRLE   pc, [pc, r12, LSL #2] ; if so, load address from jumptable
  @ B       swi_unknown           ; else handle unknown SWIs

  @ DCD     SwiHandler0           ; pointer to SWI &0 handler code
  @ DCD     SwiHandler1           ; pointer to SWI &1 handler code
  @ DCD     SwiHandler2           ; pointer to SWI &2 handler code
  @ DCD     SwiHandler3           ; pointer to SWI &3 handler code
  @ ...etc...

  @ .swihandler0
  @ ; code to handle SWI &0
  @ ...do stuff...

  @ LDMFD   sp!, {r12}            ; restore previously saved R12
  @ MOVS    pc, r14               ; restore CPSR and PC to return from SWI

  ldmia sp!, {r5, r6, lr}
  bx lr

vector_syscalls:
  .word syscall_zero
  .word syscall_one
vector_syscalls_end:
