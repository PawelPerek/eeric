#[macro_export]
macro_rules! vtype {
    (e8, $m:ident, $t:ident, $v:ident) => {
        (0b000 << 3)  | (vtype!(@m $m) << 0) | (vtype!(@t $t) << 6) | (vtype!(@v $v) << 7)
    };
    (e16, $m:ident, $t:ident, $v:ident) => {
        (0b001 << 3) | (vtype!(@m $m) << 0) | (vtype!(@t $t) << 6) | (vtype!(@v $v) << 7)
    };
    (e32, $m:ident, $t:ident, $v:ident) => {
        (0b010 << 3) | (vtype!(@m $m) << 0) | (vtype!(@t $t) << 6) | (vtype!(@v $v) << 7)
    };
    (e64, $m:ident, $t:ident, $v:ident) => {
        (0b011 << 3) | (vtype!(@m $m) << 0) | (vtype!(@t $t) << 6) | (vtype!(@v $v) << 7)
    };
    (@m m1) => { 0b000 };
    (@m m2) => { 0b001 };
    (@m m4) => { 0b010 };
    (@m m8) => { 0b011 };
    (@m mf2) => { 0b111 };
    (@m mf4) => { 0b110 };
    (@m mf8) => { 0b101 };
    (@t ta) => { 0b1 };
    (@t tu) => { 0b0 };
    (@v ma) => { 0b1 };
    (@v mu) => { 0b0 };
}

#[macro_export]
macro_rules! fuse {
    ($($instr:expr),+ $(,)? ) => {
        Instruction::Fusion(vec![$($instr),+].into_boxed_slice())
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn vtype_works() {
        assert_eq!(vtype!(e8, m2, ta, ma), 0b1_1_000_001);
        assert_eq!(vtype!(e16, m1, tu, mu), 0b0_0_001_000);
    }
}
