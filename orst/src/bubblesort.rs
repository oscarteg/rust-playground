use super::Sorter;
use crate::sort;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i, i);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn test_bubblesort() {
    let mut slice = vec![3, 2, 1];
    sort::<_, BubbleSort>(&mut slice);
    assert_eq!(slice, &[1, 2, 3])
}
