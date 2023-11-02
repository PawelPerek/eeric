.text
main:
    li a0, 0x10          # Destination address
    la a1, to_copy       # Source address

    call strcpy
    j finish

strcpy:
    mv a2, a0            # Copy destination address to a2
loop:
    lb t0, 0(a1)         # Load byte from source address
    sb t0, 0(a2)         # Store byte to destination address
    addi a1, a1, 1       # Increment source address
    addi a2, a2, 1       # Increment destination address
    bnez t0, loop        # Repeat if the loaded byte is not null (0)
    ret                  # Return

finish:

.data
to_copy:
    .asciz "Hello, world!"
