
    .section .init
    .global _start
_start:
    lui  sp, %hi(16 * 1024)
    addi sp, sp, %lo(16 * 1024)

    jal ra, main

    ebreak