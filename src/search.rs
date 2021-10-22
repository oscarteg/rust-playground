use std::cmp::Ordering;
use std::fmt::Debug;

fn binary_search<T>(haystack: &[T], needle: &T) -> Option<usize>
where
    T: Ord,
{
    let mut head = 0;
    let mut last = haystack.len();

    while head < last {
        let pivot = (head + last) / 2;

        match needle.cmp(&haystack[pivot]) {
            Ordering::Equal => return Some(pivot),
            Ordering::Less => last = pivot - 1,
            Ordering::Greater => head = pivot + 1,
        }
    }
    None
}

fn linear_search<T>(v: &[T], i: &T) -> Option<usize>
where
    T: Ord,
{
    for (index, item) in v.iter().enumerate() {
        if item == i {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::search::{binary_search, linear_search};

    #[test]
    fn test_binary_search() {
        let mut slice = vec![1, 5, 10, 25, 56, 32, 299];

        let index = binary_search(&slice, &5);
        assert_eq!(index, Some(1));
    }

    #[test]
    fn test_linear_search() {
        let mut slice = vec![1, 5, 10, 25, 56, 32, 299];

        let index = linear_search(&slice, &5);
        assert_eq!(index, Some(1));
    }
}
