.section ".text.boot"

.global _start

_start:
    // kill all existing cores except #0
    mrc p15, #0, r1, c0, c0, #5
    and r1, r1, #3
    cmp r1, #0
    bne halt

    // linker.ld puts the kernel at $8000, so we use this bottom 32KB as stack.
    mov sp, #0x8000

    // clear BSS, 16 bytes at a time
    ldr r4, =__bss_start
    ldr r9, =__bss_end
    mov r5, #0
    mov r6, #0
    mov r7, #0
    mov r8, #0
    b       2f

1:
    stmia r4!, {r5-r8}

2:
    cmp r4, r9
    blo 1b

    ldr r3, =kernel_main
    blx r3

halt:
    wfi
    b halt


// arm calling convention:
//   - r0 - r3: arguments, return values, scratch
//   - r4 - r11: callee save
