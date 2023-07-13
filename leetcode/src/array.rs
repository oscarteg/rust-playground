use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return nums.len() as i32;
        }

        let mut k = 2;

        for i in 2..nums.len() {
            if nums[i] != nums[k - 1] || nums[k - 1] != nums[k - 2] {
                nums[k] = nums[i];
                k += 1;
            }
        }

        k as i32
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]),
            7
        );
    }
}
