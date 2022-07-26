    .section .text.entry 
    .globl _start 
_start: 
    lui t0, %hi(boot_page_table)
    li t1, 0xffffffff00000000
    sub t0, t0, t1
    srli t0, t0, 12
    li t1, (8 << 60)
    or t0, t0, t1
    csrw satp, t0
    sfence.vma

    lui sp, %hi(boot_stack_top)
    addi sp, sp, %lo(boot_stack_top)
    lui t0, %hi(rust_main)
    addi t0, t0, %lo(rust_main)
    jr t0

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