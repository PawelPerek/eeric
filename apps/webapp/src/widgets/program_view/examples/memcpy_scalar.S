.text
main:
    li a0, 0x10       # Destination address
    la a1, to_copy    # Source address
    li a2, 10         # Number of bytes to copy

    call memcpy
    j finish

memcpy:
    mv a3, a0         # Copy destination address to a3
loop:
    lb t0, 0(a1)      # Load byte from source address
    sb t0, 0(a3)      # Store byte to destination address
    addi a1, a1, 1    # Increment source address
    addi a3, a3, 1    # Increment destination address
    addi a2, a2, -1   # Decrement byte count
    bnez a2, loop     # Repeat if there are more bytes to copy
    ret               # Return

finish:

.data
to_copy:
    .asciz "Hello, world!"
