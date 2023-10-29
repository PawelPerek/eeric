    .text
    .balign 4
    .global arr_sum
# uint32_t *arr_sum(uint32_t* arr, size_t n)
# a0=arr, a1=n
    
arr_sum:
    vsetvli t0, x0, e32, m8, ta, ma
    vmv.v.i v8, 0
loop:
    vsetvli t0, a1, e32, m8, ta, ma     
    vle32.v v16, (a0)

    vsetvli x0, x0, e32, m8, tu, ma 
    vadd.vv v8, v8, v16 

    sh2add a0, a0, t0
    sub a1, a1, t0 
    bnez a1, loop     
    
    vsetvli t0, x0, e32, m8, tu, ma
    vmv.s.x v0, x0
    vredsum.vs v0, v0, v8
    vmv.x.s a0, v0

    ret