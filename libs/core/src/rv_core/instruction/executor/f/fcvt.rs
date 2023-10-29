use crate::rv_core::instruction::executor::prelude::*;

pub fn ws(R { rd, rs1, rs2: _ }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);

    x[rd] = unsafe { fs1.to_int_unchecked::<i32>() as u64 };
}

pub fn wus(R { rd, rs1, rs2: _ }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);

    x[rd] = unsafe { fs1.to_int_unchecked::<u32>() as u64 };
}

pub fn sw(R { rd, rs1, rs2: _ }: R, x: &IntegerRegisters, f: &mut FloatRegisters) {
    let (_, rest) = decompose(f[rd]);

    f[rd] = compose(x[rs1] as i32 as f32, rest);
}

pub fn swu(R { rd, rs1, rs2: _ }: R, x: &IntegerRegisters, f: &mut FloatRegisters) {
    let (_, rest) = decompose(f[rd]);

    f[rd] = compose(x[rs1] as u32 as f32, rest);
}

pub fn ls(R { rd, rs1, rs2: _ }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);

    x[rd] = unsafe { fs1.to_int_unchecked::<i64>() as u64 };
}

pub fn lus(R { rd, rs1, rs2: _ }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);

    x[rd] = unsafe { fs1.to_int_unchecked() };
}

pub fn sl(R { rd, rs1, rs2: _ }: R, x: &IntegerRegisters, f: &mut FloatRegisters) {
    let (_, rest) = decompose(f[rd]);

    f[rd] = compose(x[rs1] as i64 as f32, rest);
}

pub fn slu(R { rd, rs1, rs2: _ }: R, x: &IntegerRegisters, f: &mut FloatRegisters) {
    let (_, rest) = decompose(f[rd]);

    f[rd] = compose(x[rs1] as f32, rest);
}
