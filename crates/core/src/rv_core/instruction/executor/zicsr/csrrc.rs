use crate::rv_core::instruction::executor::prelude::*;

pub fn csrrc(
    Csrr { rd, rs1, csr }: Csrr,
    x: &mut IntegerRegisters,
    c: &mut CsrRegisters,
) -> Result<(), String> {
    let csr_value = c[csr].read();
    x[rd] = csr_value;

    if rs1 != ZERO {
        let clear_mask = x[rs1];
        c[csr].write(csr_value & !clear_mask)?;
    }

    Ok(())
}
