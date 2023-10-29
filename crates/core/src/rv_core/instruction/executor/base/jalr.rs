use crate::rv_core::instruction::executor::prelude::*;

pub fn jalr(I { rd, rs1, imm12 }: I, x: &mut IntegerRegisters, pc: &mut u64) {
    x[rd] = *pc;
    *pc = x[rs1].wrapping_add(imm12 as u64).wrapping_sub(4) & !1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jalr_works() {
        let mut x = IntegerRegisters::new(&Memory::default());
        let mut pc = 8;
        x[10] = 16;

        jalr(
            I {
                rd: 5,
                rs1: 10,
                imm12: -4,
            },
            &mut x,
            &mut pc,
        );

        assert_eq!(x[5], 8);
        assert_eq!(pc, 8);
    }
}
