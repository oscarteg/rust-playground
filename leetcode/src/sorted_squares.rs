fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut location = nums.len();

    // Pre-allocate the output array so we can index from the end, why from the end?
    let mut output = vec![0; location];

    // Left pointer
    let mut left = 0_usize;
    // Right pointer
    let mut right = nums.len() - 1;

    // Square both pointers
    let mut computed_left = i32::pow(nums[left], 2);
    let mut computed_right = i32::pow(nums[right], 2);

    while left != right {
        // Move the location on spot to the left
        // [-4_left, -1, 0, 3_location, 1_right]
        location -= 1;

        // Because we start with an desc sorted array we only have to check the 2 pointers and not the other values
        if computed_left > computed_right {
            // Put the calculated in the locations place
            output[location] = computed_left;
            // Move the pointer 1 to the right
            left += 1;
            // Recalculate
            computed_left = i32::pow(nums[left], 2);
        } else {
            output[location] = computed_right;
            // Move the pointer 1 to the left
            right -= 1;
            // Recalculate
            computed_right = i32::pow(nums[right], 2);
        }
    }
    output[0] = computed_left;
    output
}

pub fn sorted_squares_short(mut nums: Vec<i32>) -> Vec<i32> {
    // I have no idea how the fuck this works
    nums.sort_unstable_by_key(|x| x.abs());
    nums.iter_mut().for_each(|x| *x = x.pow(2));
    nums
}

#[cfg(test)]
mod tests {
    use crate::sorted_squares::{sorted_squares, sorted_squares_short};

    #[test]
    fn it_works_short() {
        assert_eq!(
            sorted_squares_short(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn it_works() {
        assert_eq!(
            sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn unsorted_array() {
        assert_ne!(
            sorted_squares(vec![10, -1, -3, 0, -4]),
            vec![0, 1, 9, 16, 100]
        );
    }
}
