use std::fmt::Debug;

fn binary_search<T>(haystack: &[T], needle: T) -> Result<usize, usize>
where
    T: Ord + Debug,
{
    let mut head = 0;
    let mut last = haystack.len();

    while head < last {
        let pivot = (head + last) / 2;

        if needle == haystack[pivot] {
            return Ok(pivot);
        } else if needle < haystack[pivot] {
            last = pivot - 1;
        } else {
            head = pivot + 1;
        }
    }
    Err((head + last) / 2 + 1)
}

fn linear_search<T>(v: &[T], i: T) -> Result<usize, usize>
where
    T: Ord + Debug,
{
    for (p, x) in v.iter().enumerate() {
        if *x == i {
            return Ok(p);
        } else if *x > i {
            return Err(p);
        }
    }
    Err(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let mut slice = vec![1, 5, 10, 25, 56, 32, 299];

        let index = super::binary_search(&slice, 5);
        assert_eq!(index, Ok(1));
    }
}
