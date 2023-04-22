fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() > 1 {
        let pivot_idx = partition(arr);
        let (left, right) = arr.split_at_mut(pivot_idx);
        quicksort(left);
        quicksort(&mut right[1..]);
    }
}

/// The partition function is a helper function that partitions the slice into two halves around a pivot element and returns the index of the pivot element after partitioning.
fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_idx = len / 2;
    // Swap pivot with last
    arr.swap(pivot_idx, len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

#[cfg(test)]
mod tests {
    use crate::quicksort::quicksort;

    #[test]
    fn test_empty() {
        let mut arr: [i32; 0] = [];
        quicksort(&mut arr);
        assert_eq!(&arr[..], &[]);
    }

    #[test]
    fn test_single() {
        let mut arr = [1];
        quicksort(&mut arr);
        assert_eq!(&arr[..], &[1]);
    }

    #[test]
    fn test_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        quicksort(&mut arr);
        assert_eq!(&arr[..], &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse() {
        let mut arr = [5, 4, 3, 2, 1];
        quicksort(&mut arr);
        assert_eq!(&arr[..], &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random() {
        let mut arr = [5, 1, 4, 2, 3];
        quicksort(&mut arr);
        assert_eq!(&arr[..], &[1, 2, 3, 4, 5]);
    }
}
