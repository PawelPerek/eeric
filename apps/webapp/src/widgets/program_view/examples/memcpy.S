main:
    li a0, 0x10
    la a1, to_copy
    li a2, 10

    j memcpy
    # j finish

memcpy:
    j scalar
    # j vector
    ret

scalar:
    beqz    a2, end_memcopy

sloop:
    lbu     t0, 0(a1)
      addi    a1, a1, 1
      addi    a0, a0, 1
    sb      t0, 0(a0)
      addi    a2, a2, -1
      bnez    a2, sloop

end_memcopy:
    ret

vector:
    mv a3, a0 # Copy destination
vloop:
  vsetvli t0, a2, e8, m8, ta, ma   # Vectors of 8b
  vle8.v v0, (a1)                  # Load bytes
    add a1, a1, t0                 # Bump pointer
    sub a2, a2, t0                 # Decrement count
  vse8.v v0, (a3)                  # Store bytes
    add a3, a3, t0                 # Bump pointer
    bnez a2, vloop                  # Any more?
    ret                            # Return
    
finish:
    
.data
to_copy:
    .asciz "Hello, world!"