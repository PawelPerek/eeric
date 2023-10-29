use crate::rv_core::instruction::executor::prelude::*;

pub fn csrrsi(
    Csri { rd, uimm, csr }: Csri,
    x: &mut IntegerRegisters,
    c: &mut CsrRegisters,
) -> Result<(), String> {
    let csr_value = c[csr].read();
    x[rd] = csr_value;

    let clear_mask = uimm as u64;
    c[csr].write(csr_value | clear_mask)?;

    Ok(())
}
