use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    ///
    /// A O(n) algorithm for sorting
    ///
    /// # Arguments
    ///
    /// * `slice`:
    ///
    /// returns: void
    ///
    /// # Examples
    ///
    /// ```
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        // This is to check if everything is in order
        let mut swapped = true;
        while swapped {
            // Assume everything is in order
            swapped = false;
            // Loop from index 1 until the last in the array
            for i in 1..slice.len() {
                // Look at the previous value in the array (because we start from 1 we can do that)
                // And look at the current value
                // If the previous value is larger, we need to swap
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    // We have swapped a value so we need to run the whole code again to check the array
                    swapped = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bubblesort::BubbleSort;
    use crate::Sorter;

    #[test]
    fn test_bubblesort() {
        let mut slice = vec![3, 2, 1];
        BubbleSort::sort(&mut slice);
        // sort::<_, BubbleSort>(&mut slice);
        assert_eq!(slice, &[1, 2, 3])
    }

    #[test]
    fn test_when_everything_is_already_in_order() {
        let mut slice = vec![1, 2, 3];
        BubbleSort::sort(&mut slice);
        // sort::<_, BubbleSort>(&mut slice);
        assert_eq!(slice, &[1, 2, 3])
    }
}
