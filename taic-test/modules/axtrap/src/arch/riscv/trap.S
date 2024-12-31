.macro SAVE_REGS, from_user
    addi    sp, sp, -{trapframe_size}
    PUSH_GENERAL_REGS

    csrr    t0, sepc
    csrr    t1, sstatus
    csrrw   t2, sscratch, zero          // save sscratch (sp) and zero it
    STR     t0, sp, 31                  // tf.sepc
    STR     t1, sp, 32                  // tf.sstatus
    STR     t2, sp, 1                   // tf.regs.sp
    .short  0xa622                      // fsd fs0,264(sp)
    .short  0xaa26                      // fsd fs1,272(sp)
.if \from_user == 1
    csrr    t0, 0x041
    STR     t0, sp, 35                  // tf.uepc
    csrr    t0, 0x000
    STR     t0, sp, 36                  // tf.ustatus
    csrr    t0, 0x005
    STR     t0, sp, 37                  // tf.utvec
    csrr    t0, 0x043
    STR     t0, sp, 38                  // tf.utval
    csrr    t0, 0x042
    STR     t0, sp, 39                  // tf.ucause
    csrr    t0, 0x044
    STR     t0, sp, 40                  // tf.uip
    csrr    t0, 0x004
    STR     t0, sp, 41                  // tf.uie

    LDR     t1, sp, 2                   // load user gp with CPU ID
    LDR     t0, sp, 3                   // load supervisor tp
    STR     gp, sp, 2                   // save user gp and tp
    STR     tp, sp, 3
    mv      gp, t1
    mv      tp, t0
.endif
.endm

.macro RESTORE_REGS, from_user
.if \from_user == 1
    LDR     t0, sp, 35                  // tf.uepc
    csrw    0x041, t0
    LDR     t0, sp, 36                  // tf.ustatus
    csrw    0x000, t0
    LDR     t0, sp, 37                  // tf.utvec
    csrw    0x005, t0
    LDR     t0, sp, 38                  // tf.utval
    csrw    0x043, t0
    LDR     t0, sp, 39                  // tf.ucause
    csrw    0x042, t0
    LDR     t0, sp, 40                  // tf.uip
    csrw    0x044, t0
    LDR     t0, sp, 41                  // tf.uie
    csrw    0x004, t0

    LDR     t1, sp, 2
    LDR     t0, sp, 3
    STR     gp, sp, 2                   // load user gp and tp
    STR     tp, sp, 3                   // save supervisor tp
    mv      gp, t1
    mv      tp, t0
    addi    t0, sp, {trapframe_size}    // put supervisor sp to scratch
    csrw    sscratch, t0
.endif

    LDR     t0, sp, 31
    LDR     t1, sp, 32
    csrw    sepc, t0
    csrw    sstatus, t1
    .short  0x2432                      // fld fs0,264(sp)
    .short  0x24d2                      // fld fs1,272(sp)
    POP_GENERAL_REGS
    LDR     sp, sp, 1                   // load sp from tf.regs.sp
.endm

.section .text
.balign 4
.global trap_vector_base
trap_vector_base:
    // sscratch == 0: trap from S mode
    // sscratch != 0: trap from U mode
    csrrw   sp, sscratch, sp            // switch sscratch and sp
    bnez    sp, .Ltrap_entry_u

    csrr    sp, sscratch                // put supervisor sp back
    j       .Ltrap_entry_s

.Ltrap_entry_s:
    SAVE_REGS 0
    mv      a0, sp
    li      a1, 0
    call    riscv_trap_handler
    RESTORE_REGS 0
    sret

.Ltrap_entry_u:
    SAVE_REGS 1
    mv      a0, sp
    li      a1, 1
    call    riscv_trap_handler
    RESTORE_REGS 1
    sret
