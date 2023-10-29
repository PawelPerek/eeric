use crate::rv_core::instruction::executor::prelude::*;

pub fn beq(S { rs1, rs2, imm12 }: S, x: &IntegerRegisters, pc: &mut u64) {
    if x[rs1] == x[rs2] {
        *pc = pc.wrapping_add(imm12 as u64).wrapping_sub(4);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beq_works() {
        let x = IntegerRegisters::new(&Memory::default());
        let mut pc = 8;

        beq(
            S {
                rs1: 0,
                rs2: 0,
                imm12: -8,
            },
            &x,
            &mut pc,
        );

        assert_eq!(pc, -4_i64 as u64);
    }
}
