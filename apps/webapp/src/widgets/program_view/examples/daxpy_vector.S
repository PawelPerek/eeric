main:
    li a0, 4
    la t0, fp
    fld fa0, 0(t0)
    la a1, x
    la a2, y

    call daxpy
    j finish

daxpy:
    vsetvli a4, a0, e64, m8, ta, ma
    vse64.v v0, (a1)
    sub a0, a0, a4
    slli a4, a4, 2
    add a1, a1, a4
    vse64.v v8, (a2)
    vfmacc.vf v8, fa0, v0
    vse64.v v8, (a2)
    add a2, a2, a4
    bnez a0, daxpy
    ret

finish:

.data
fp:
    .double 3.14159265
x:
    .double 0.84, 0.14, 0.36, 0.42
y:
    .double 0.61, 0.56, 0.77, 0.60