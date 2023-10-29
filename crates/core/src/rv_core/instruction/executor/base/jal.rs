use crate::rv_core::instruction::executor::prelude::*;

pub fn jal(U { rd, imm20 }: U, x: &mut IntegerRegisters, pc: &mut u64) {
    x[rd] = *pc;
    *pc = pc.wrapping_add(imm20 as u64).wrapping_sub(4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jal_works() {
        let mut x = IntegerRegisters::new(&Memory::default());
        let mut pc = 8;

        jal(U { rd: 5, imm20: -12 }, &mut x, &mut pc);

        assert_eq!(x[5], 8);
        assert_eq!(pc, -8_i64 as u64);
    }
}
