    .globl _start
    .text

_start:
    # Create a stack
    li.d $sp, 0x98
    slli.d $sp, $sp, 24

    # Call rust_main
    la $ra, rust_main
    jr $ra

    .end