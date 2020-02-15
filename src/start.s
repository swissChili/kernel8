.section ".text._start"

.global _start

_start:
    mrs     x1, mpidr_el1 // multiproc reg
    and     x1, x1, #3
    cbz     x1, 2f
1:  wfe
    b       1b // halt the cpu if an event occurs
2:
    ldr     x1, =_start
    mov     sp, x1
    bl      main // jump to the kernel
    b       1b
