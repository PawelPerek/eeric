use crate::rv_core::instruction::executor::prelude::*;

pub fn csrrwi(
    Csri { rd, uimm, csr }: Csri,
    x: &mut IntegerRegisters,
    c: &mut CsrRegisters,
) -> Result<(), String> {
    let csr_value = c[csr].read();
    c[csr].write(uimm as u64)?;
    x[rd] = csr_value;

    Ok(())
}
