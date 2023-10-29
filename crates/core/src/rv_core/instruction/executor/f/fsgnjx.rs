use crate::rv_core::instruction::executor::prelude::*;

pub fn s(R { rd, rs1, rs2 }: R, f: &mut FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);
    let (fs2, _) = decompose(f[rs2]);

    let value = match (fs1.is_sign_positive(), fs2.is_sign_positive()) {
        (true, true) => fs1,   // +x, +y => +x
        (true, false) => -fs1, // +x, -y => -x
        (false, true) => -fs1, // -x, +y => -x
        (false, false) => fs1, // -x, -y => +x
    };

    let (_, rest) = decompose(f[rd]);

    f[rd] = compose(value, rest);
}
