    .section .text.entry 
    .globl _start 
_start: 
    la sp, boot_stack_top 
    call rust_main 

    .section .bss.stack 
    .globl boot_stack
boot_stack: 
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: 

    .section .data 
    .align 12
boot_page_table: 
    .quad 0 
    .quad 0 
    .quad (0x80000 << 10) | 0xcf
    .zero 507 * 8
    .quad (0x80000 << 10) | 0xcf 
    .quad 0 