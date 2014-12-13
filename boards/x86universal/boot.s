.set ALIGN,     1<<0
.set MEMINFO,   1<<1
.set FLAGS,     ALIGN | MEMINFO
.set MAGIC,     0x1badb002
.set CHECKSUM,  -(MAGIC + FLAGS)

.section .boot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

stack_bottom:
.skip 4096
stack_top:

.section .text
.global _start
.type _start, @function
_start:
    mov $stack_top, %esp
    call kentry

