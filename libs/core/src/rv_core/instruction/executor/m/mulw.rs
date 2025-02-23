use crate::rv_core::instruction::executor::prelude::*;

pub fn mulw(R { rd, rs1, rs2 }: R, x: &mut IntegerRegisters) {
    let result = (x[rs1] as i32).wrapping_mul(x[rs2] as i32);
    x[rd] = result as i64 as u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mulw_works() {
        let mut x = IntegerRegisters::new(&Memory::default());

        x[A1] = 0xb504f334;
        x[A2] = -2i64 as u64;

        mulw(
            R {
                rd: A0,
                rs1: A1,
                rs2: A2
            },
            &mut x
        );

        assert_eq!(x[A0], 0xffffffff95f61998);
    }
}