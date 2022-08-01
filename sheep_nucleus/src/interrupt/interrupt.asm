.altmacro 
.set REG_SIZE, 8
# Context 大小设置
.set CONTEXT_SIZE, 34 

.macro SAVE reg, offset
    sd \reg, \offset*8(sp)
.endm

.macro SAVE_N n
    SAVE x\n, \n
.endm 

.macro LOAD reg, offset 
    ld \reg, \offset*8(sp) 
.endm

.macro LOAD_N n 
    LOAD x\n, \n
.endm 

    .section .text 
    .globl __interrupt 

# 保存中断
# 保存 Context 并且进入 Rust 中的中断处理函数 interrupt::handler::handle_interrupt() 

__interrupt: 
    csrrw sp, sscratch, sp
    # 在栈上开辟 Context 所需的空间
    addi sp, sp, -34 * 8

    # 保存通用寄存器，除了 x0 （固定为零）
    SAVE x1, 1 
    # 将原来的 sp（sp 又名 x2）写入 2 位置
    addi x1, sp, 34*8 
    SAVE x1, 2
    # 保存 x3 至 x31. 
    .set n, 3 
    .rept 29
        SAVE_N %n 
        .set n, n+1
    .endr 

    # 取出 csr 并保存
    csrr s1, sstatus 
    csrr s2, sepc 
    SAVE s1, 32 
    SAVE s2, 33 

    # 调用 handle_interrupt, 传入参数
    # context: &mut Context 
    mv a0, sp 
    # scause: Scause 
    csrr a1, scause 
    # stval: usize 
    csrr a2, stval 
    
    # jal handle_interrupt
    jal handle_interrupt_backup

    .globl __restore 
# 离开中断
# 从 Context 中恢复所有寄存器，并跳转至 Context 中的 sepc 位置
__restore: 
    mv sp, a0
    # 恢复 csr 
    LOAD s1, 32 
    LOAD s2, 33 
    csrw sstatus, s1 
    csrw sepc, s2

    # 将内核栈地址写入 sscratch
    # addi    t0, sp, CONTEXT_SIZE * REG_SIZE
    # csrw    sscratch, t0 

    # 恢复通用寄存器
    LOAD x1, 1
    # 恢复 x3 至 x31 
    .set n, 3 
    .rept 29 
        LOAD_N %n 
        .set n, n + 1
    .endr 

    LOAD x2, 2

    csrrw sp, sscratch, sp 
    
    sret 