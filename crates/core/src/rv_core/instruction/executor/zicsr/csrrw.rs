use crate::rv_core::instruction::executor::prelude::*;

pub fn csrrw(
    Csrr { rd, rs1, csr }: Csrr,
    x: &mut IntegerRegisters,
    c: &mut CsrRegisters,
) -> Result<(), String> {
    let csr_value = c[csr].read();
    c[csr].write(x[rs1])?;
    x[rd] = csr_value;

    Ok(())
}
