macro_rules! masked_map {
    ($func:expr, $mask_iter:expr, $($iters:expr),+ ) => {{
        use ::itertools::izip;

        izip!($mask_iter, izip!($($iters),+)).map(move |(m, values)| {
            if m == 1 {
                $func((m, values))
            } else {
                values.0
            }
        })
    }};
}

pub(crate) use masked_map;

#[cfg(test)]
mod tests {
    #[test] 
    fn masked_map() {
        let mask = vec![1, 0, 1, 0, 1, 0, 1, 0];
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let result: Vec<_> = masked_map!(
            |(mask, values): (u64, (u64, u64))| values.0 + values.1, 
            mask.into_iter(), 
            values.iter().cloned(), 
            values.iter().cloned()
        ).collect();

        assert_eq!(result, vec![2, 2, 8, 4, 14, 6, 22, 8]);
    }
}
