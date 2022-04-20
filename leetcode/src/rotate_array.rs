// Given an array, rotate the array to the right by k steps, where k is non-negative.

// Input: nums = [1,2,3,4,5,6,7], k = 3
// Output: [5,6,7,1,2,3,4]
// Explanation:
// rotate 1 steps to the right: [7,1,2,3,4,5,6]
// rotate 2 steps to the right: [6,7,1,2,3,4,5]
// rotate 3 steps to the right: [5,6,7,1,2,3,4]

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    // let mut k = k as usize % nums.len();
    // nums.rotate_right(k as usize)
    let m = k as usize % nums.len();

    println!("{m:?}");
    nums.reverse();
    nums[m..].reverse();
    nums[..m].reverse();
}

#[cfg(test)]
mod tests {
    use crate::rotate_array::rotate;

    #[test]
    fn it_works() {
        let mut example = vec![1, 2, 3, 4, 5, 6, 7];

        rotate(&mut example, 3);

        let expected = vec![5, 6, 7, 1, 2, 3, 4];

        assert_eq!(example, expected);
    }
}
