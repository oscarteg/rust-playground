fn heapify<T>(array: &mut [T])
where
    T: Ord,
{
    let last_parent = (array.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        move_down(array, i);
    }
}

pub fn heap_sort<T>(array: &mut [T])
where
    T: Ord,
{
    // Base case
    if array.len() <= 1 {
        return;
    }

    heapify(array);

    for end in (1..array.len()).rev() {
        array.swap(0, end);
        move_down(&mut array[..end], 0);
    }
}

fn move_down<T>(array: &mut [T], root: usize)
where
    T: Ord,
{
    let last = array.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }

        let right = left + 1;
        let max = if right <= last && array[right] > array[left] {
            right
        } else {
            left
        };

        if array[max] > array[root] {
            array.swap(root, max);
        }
        root = max;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty() {
        let mut array: Vec<i32> = Vec::new();
        heap_sort(&mut array);
        assert_eq!(&array, &[]);
    }

    #[test]
    fn unsorted_array() {
        let mut array = vec![2, 4, 6, 1];
        heap_sort(&mut array);
        assert_eq!(&array, &[1, 2, 4, 6]);
    }

    #[test]
    fn unsorted_array_characters() {
        let mut array = vec!['g', 'd', 'o'];
        heap_sort(&mut array);
        assert_eq!(&array, &['d', 'g', 'o']);
    }
}
