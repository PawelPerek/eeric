use crate::rv_core::instruction::executor::prelude::*;

pub fn sd(R { rd, rs1, rs2: _ }: R, f: &mut FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);

    f[rd] = fs1 as f64;
}

pub fn ds(R { rd, rs1, rs2: _ }: R, f: &mut FloatRegisters) {
    let (fs1, _) = decompose(f[rs1]);
    let (_, rest) = decompose(f[rd]);

    f[rd] = compose(fs1, rest);
}

pub fn wd(R { rd, rs1, rs2: _ }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    x[rd] = unsafe { f[rs1].to_int_unchecked::<i32>() as u64 };
}

pub fn wud(R { rd, rs1, rs2: _ }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    x[rd] = unsafe { f[rs1].to_int_unchecked::<u32>() as i32 as u64 };
}

pub fn dw(R { rd, rs1, rs2: _ }: R, x: &IntegerRegisters, f: &mut FloatRegisters) {
    f[rd] = x[rs1] as i32 as f64;
}

pub fn dwu(R { rd, rs1, rs2: _ }: R, x: &IntegerRegisters, f: &mut FloatRegisters) {
    f[rd] = x[rs1] as u32 as f64;
}

pub fn ld(R { rd, rs1, rs2: _ }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    x[rd] = unsafe { f[rs1].to_int_unchecked::<i64>() as u64 };
}

pub fn lud(R { rd, rs1, rs2: _ }: R, x: &mut IntegerRegisters, f: &FloatRegisters) {
    x[rd] = unsafe { f[rs1].to_int_unchecked() };
}

pub fn dl(R { rd, rs1, rs2: _ }: R, x: &IntegerRegisters, f: &mut FloatRegisters) {
    f[rd] = x[rs1] as i64 as f64;
}

pub fn dlu(R { rd, rs1, rs2: _ }: R, x: &IntegerRegisters, f: &mut FloatRegisters) {
    f[rd] = x[rs1] as f64;
}
