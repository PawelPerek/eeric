.text
main:
    la a0, to_copy       # Source address

    call strlen
    j finish

strlen:
    mv a1, a0            # Copy source address to a1
loop:
    lb t0, 0(a1)         # Load byte from source address
    addi a1, a1, 1       # Increment source address
    bnez t0, loop        # Repeat if the loaded byte is not null (0)
    sub a0, a1, a0       # Calculate length by subtracting start address from end address
    addi a0, a0, -1      # Subtract 1 as a1 is one past the null byte

    ret                  # Return

finish:

.data
to_copy:
    .asciz "Hello, world!"
