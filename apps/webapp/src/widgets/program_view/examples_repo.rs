use super::Example;

static EXAMPLES_REPOSITORY: &[(Example, &'static str)] = &[
    (
        Example::Memcpy,
        r#"main:
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
    .asciz "Hello, world!""#,
    ),
    (
        Example::Strcpy,
        r#"strcpy:
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

    ret"#,
    ),
    (
        Example::Strncpy,
        r#"strncpy:
    mv a3, a0             # Copy dst
loop:
  vsetvli x0, a2, e8, m8, ta, ma   # Vectors of bytes.
  vle8ff.v v8, (a1)        # Get src bytes
  vmseq.vi v1, v8, 0      # Flag zero bytes
    csrr t1, vl           # Get number of bytes fetched
  vfirst.m a4, v1         # Zero found?
  vmsbf.m v0, v1          # Set mask up to before zero byte.
  vse8.v v8, (a3), v0.t    # Write out non-zero bytes
    bgez a4, zero_tail    # Zero remaining bytes.
    sub a2, a2, t1        # Decrement count.
    add a3, a3, t1        # Bump dest pointer
    add a1, a1, t1        # Bump src pointer
    bnez a2, loop         # Anymore?

    ret

zero_tail:
  sub a2, a2, a4          # Subtract count on non-zero bytes.
  add a3, a3, a4          # Advance past non-zero bytes.
  vsetvli t1, a2, e8, m8, ta, ma   # Vectors of bytes.
  vmv.v.i v0, 0           # Splat zero.

zero_loop:
  vse8.v v0, (a3)          # Store zero.
    sub a2, a2, t1        # Decrement count.
    add a3, a3, t1        # Bump pointer
    vsetvli t1, a2, e8, m8, ta, ma   # Vectors of bytes.
    bnez a2, zero_loop    # Anymore?

    ret"#,
    ),
    (
        Example::Strlen,
        r#"strlen:
    mv a3, a0             # Save start
loop:
    vsetvli a1, x0, e8, m8, ta, ma  # Vector of bytes of maximum length
    vle8ff.v v8, (a3)      # Load bytes
    csrr a1, vl           # Get bytes read
    vmseq.vi v0, v8, 0    # Set v0[i] where v8[i] = 0
    vfirst.m a2, v0       # Find first set bit
    add a3, a3, a1        # Bump pointer
    bltz a2, loop         # Not found?

    add a0, a0, a1        # Sum start + bump
    add a3, a3, a2        # Add index
    sub a0, a3, a0        # Subtract start address+bump

    ret"#,
    ),
    (
        Example::Saxpy,
        r#"saxpy:
    vsetvli a4, a0, e32, m8, ta, ma
    vle32.v v0, (a1)
    sub a0, a0, a4
    slli a4, a4, 2
    add a1, a1, a4
    vle32.v v8, (a2)
    vfmacc.vf v8, fa0, v0
    vse32.v v8, (a2)
    add a2, a2, a4
    bnez a0, saxpy
    ret"#,
    ),
];

pub fn get_example(example: Example) -> &'static str {
    EXAMPLES_REPOSITORY
        .iter()
        .find(|&&(id, _)| id == example)
        .unwrap()
        .1
}
