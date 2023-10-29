use crate::rv_core::instruction::executor::prelude::*;

pub fn vsetvl(
    Vsetvl { rd, rs1, rs2 }: Vsetvl,
    x: &mut IntegerRegisters,
    v: &mut VectorContext<'_>,
) {
    v.set_vtype(x[rs2]).unwrap();

    let avl = match (rd, rs1) {
        (ZERO, ZERO) => v.csr[VL].read(),
        (_, ZERO) => v.vlmax() as u64,
        (_, rs1) => x[rs1].min(v.vlmax() as u64),
    };

    unsafe { v.csr[VL].set(avl) };
    x[rd] = avl;
}
