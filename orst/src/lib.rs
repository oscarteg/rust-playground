trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice)
}

mod bubblesort;

#[cfg(test)]
mod tests {
    use crate::{sort, Sorter};

    struct StdSorter;

    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }
    #[test]
    fn std_works() {
        let mut things = vec![5, 2, 1];
        sort::<_, StdSorter>(&mut things);
        assert_eq!(things, &[1, 2, 5]);
    }
}
