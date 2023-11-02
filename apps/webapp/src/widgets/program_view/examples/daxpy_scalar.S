.text
main:
    li a0, 4          # n = 4
    la t0, fp         # Address of scalar a
    fld fa0, 0(t0)    # Load scalar a into fa0
    la a1, x          # Address of vector x
    la a2, y          # Address of vector y

    call daxpy
    j finish

daxpy:
    li t1, 0          # i = 0
loop:
    fld fa1, 0(a1)    # Load x[i] into fa1
    fmul.d fa1, fa1, fa0 # Compute a*x[i]
    fld fa2, 0(a2)    # Load y[i] into fa2
    fadd.d fa2, fa2, fa1 # Compute y[i] = y[i] + a*x[i]
    fsd fa2, 0(a2)    # Store result back to y[i]
    addi t1, t1, 1    # i = i + 1
    addi a1, a1, 8    # Move to next element of x
    addi a2, a2, 8    # Move to next element of y
    blt t1, a0, loop  # If i < n, repeat
    ret               # Return

finish:

.data
fp:
    .double 3.14159265
x:
    .double 0.84, 0.14, 0.36, 0.42
y:
    .double 0.61, 0.56, 0.77, 0.60
