    .text
    .globl   memcpy
    .type  memcpy, function
    .align 0
memcpy:
    loop:
        movb (%ebx), %dl
        movb %dl, (%eax)
        decl %ecx
        incl %eax
        incl %ebx
        test %ecx, %ecx
        jne loop
    ret

