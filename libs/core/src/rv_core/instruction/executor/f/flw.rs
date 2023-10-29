use crate::rv_core::instruction::executor::prelude::*;

pub fn flw(I { rd, rs1, imm12 }: I, x: &IntegerRegisters, f: &mut FloatRegisters, mem: &Memory) {
    let addr = x[rs1].wrapping_add(imm12 as u64);
    let fp = f32::from_le_bytes(mem.get(addr as usize));

    let (_, rest) = decompose(f[rd]);

    f[rd] = compose(fp, rest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_loads() {
        let memory = Memory::new((0..123).into_iter());
        let x = IntegerRegisters::new(&memory);
        let mut f = FloatRegisters::default();

        flw(
            I {
                rd: 0,
                rs1: 0,
                imm12: 0x10,
            },
            &x,
            &mut f,
            &memory,
        );

        let (float, rest) = decompose(f[FT0]);

        assert_eq!(rest, 0);
        assert_eq!(float, f32::from_bits(0x13121110));
    }

    #[test]
    fn float_loads_pi() {
        let mut memory = Memory::new((0..123).into_iter());
        let x = IntegerRegisters::new(&memory);
        let mut f = FloatRegisters::default();

        memory.assign(&[37, 6, 73, 64]);

        flw(
            I {
                rd: 0,
                rs1: 0,
                imm12: 0,
            },
            &x,
            &mut f,
            &memory,
        );

        let (float, rest) = decompose(f[FT0]);

        print!("{}", float);

        assert_eq!(rest, 0);
        assert_eq!(float, f32::from_bits(0x40490625));
    }
}
