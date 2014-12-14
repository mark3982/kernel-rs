.set ALIGN,     1<<0
.set MEMINFO,   1<<1
.set FLAGS,     ALIGN | MEMINFO
.set MAGIC,     0x1badb002
.set CHECKSUM,  -(MAGIC + FLAGS)

.section .boot
# Put the start function at the top of the image
# along with the multiboot header.
.global _start
.type _start, @function
_start:
    mov $stack_top, %esp
    call kentry
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

.section .data
# Put the stack in the data section since we are assuming
# that we will be booting up in fast writable memory.
stack_bottom:
.skip 4096
stack_top:

