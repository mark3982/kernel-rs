    .text
    .globl   memcpy
    .type  memcpy, function
    .align 0
memcpy:
    # copyin from r1 to r0 of r2 bytes
    push {r0, r1, r2, r3}
loop:
    ldrb r3, [r1]
    strb r3, [r0]
    subs r2, r2, $1
    cmp r2, $0
    bne loop
    pop {r0, r1, r2}
    mov pc, lr

