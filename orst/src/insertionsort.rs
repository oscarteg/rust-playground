use super::Sorter;

pub struct InsertionSort;

pub struct InsertionSortWithBinary;
asdasd

impl Sorter for InsertionSortWithBinary {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        // Walk all the elements
        for unsorted in 1..slice.len() {
            // use binary search to find index
            // then use .insert to splice in i

            let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                // [ a, c, e].binary_search(c) => Ok(1)
                // [ a, c, e].binary_search(b) => Err(1) // could not find but this is the index it should be
                Ok(i) => i,
                Err(i) => i,
            };

            // Difficult
            // We rotate right starting from the starting location and ending at where we are at..
            // rotate all the elements to the right and the last element goes to the beginning again.
            slice[i..=unsorted].rotate_right(1);
        }
    }
}

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        // Always 1, list of 1 is always sorted, duh
        // Walk all the elements
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            // take slice[unsorted] place in sorted location in slice[..unsorted]
            // [ 1 3 4 | 2 ]
            // [ 1 3 4 2 |  ]
            // [ 1 3 2 4 |  ]
            // [ 1 3 2 4 |  ]
            // Keep swapping until next element is smaller then current element
            let mut i = unsorted;

            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                i -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut slice = vec![3, 2, 1];
        // sort::<_, InsertionSort>(&mut slice);
        // Only when Sorter trait in scope
        InsertionSort::sort(&mut slice);
        assert_eq!(slice, &[1, 2, 3])
    }

    #[test]
    fn test_when_everything_is_already_in_order() {
        let mut slice = vec![1, 2, 3];
        // sort::<_, InsertionSort>(&mut slice);
        InsertionSort::sort(&mut slice);
        assert_eq!(slice, &[1, 2, 3])
    }

    #[test]
    fn test_with_binary_insertion_sort() {
        let mut slice = vec![3, 2, 1];
        // sort::<_, InsertionSort>(&mut slice);
        // Only when Sorter trait in scope
        InsertionSortWithBinary::sort(&mut slice);
        assert_eq!(slice, &[1, 2, 3])
    }

    #[test]
    fn test_with_binary_when_everything_is_already_in_order() {
        let mut slice = vec![1, 2, 3];
        // sort::<_, InsertionSort>(&mut slice);
        InsertionSortWithBinary::sort(&mut slice);
        assert_eq!(slice, &[1, 2, 3])
    }
}
