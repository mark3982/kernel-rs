.global _start
.section .boot
stack_bottom:
.skip 4096
stack_top:
_start:
    #mov %sp, #0x2000
    mov %sp, $stack_top
    b kentry
val0:
    .long 0x10009000


