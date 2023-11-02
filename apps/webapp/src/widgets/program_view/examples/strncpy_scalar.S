.text
main:
    li a0, 0x10        # Destination address
    la a1, to_copy     # Source address
    li a2, 9           # At most

    call strncpy
    j finish

strncpy:
    mv a3, a0          # Copy destination address to a3
    mv t0, a2          # Copy max count to t0
loop:
    lb t1, 0(a1)       # Load byte from source address
    sb t1, 0(a3)       # Store byte to destination address
    addi a1, a1, 1     # Increment source address
    addi a3, a3, 1     # Increment destination address
    addi t0, t0, -1    # Decrement count
    bnez t1, check_count # If byte is not null, check count
fill_zero:
    bnez t0, zero_loop # If count is not zero, fill with zero
    ret

check_count:
    bnez t0, loop      # If count is not zero, continue loop
    ret

zero_loop:
    sb x0, 0(a3)       # Store zero to destination address
    addi a3, a3, 1     # Increment destination address
    addi t0, t0, -1    # Decrement count
    j fill_zero        # Jump back to fill_zero

finish:

.data
to_copy:
    .asciz "Hello, world!"
