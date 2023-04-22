fn largest_product(s: &str, n: usize) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }

    s.chars()
        .map(|c| c.to_digit(10).map(u64::from))
        .collect::<Option<Vec<_>>>()
        .map(|d| d.windows(n).map(|w| w.iter().product()).max().unwrap_or(0))
}

#[cfg(test)]
mod test {
    use crate::largest_product::largest_product;

    #[test]
    fn test() {
        assert_eq!(largest_product("1027839564", 3), Some(270));
        assert_eq!(largest_product("1027839564", 5), Some(7560));
        assert_eq!(
            largest_product("73167176531330624919225119674426574742355349194934", 6),
            Some(23520)
        );
        assert_eq!(largest_product("12345", 0), Some(0));
        assert_eq!(largest_product("0", 1), Some(0));
    }
}
