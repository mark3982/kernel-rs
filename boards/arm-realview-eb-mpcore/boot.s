.global _start
.section .boot
stack_bottom:
.skip 4096
stack_top:
_start:
    ldr %sp, =stack_top
    b kentry


