.text
main:
    li a0, 0x10
    la a1, to_copy

    call strcpy
    j finish

strcpy:
    mv a2, a0             # Copy dst
    li t0, -1             # Infinite AVL
loop:
  vsetvli x0, t0, e8, m8, ta, ma  # Max length vectors of bytes
  vle8ff.v v8, (a1)        # Get src bytes
    csrr t1, vl           # Get number of bytes fetched
  vmseq.vi v1, v8, 0      # Flag zero bytes
  vfirst.m a3, v1         # Zero found?
    add a1, a1, t1        # Bump pointer
  vmsif.m v0, v1          # Set mask up to and including zero byte.
  vse8.v v8, (a2), v0.t    # Write out bytes
    add a2, a2, t1        # Bump pointer
    bltz a3, loop         # Zero byte not found, so loop

    ret

finish:

.data
to_copy:
    .asciz "Hello, world!"
