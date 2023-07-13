use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 3 {
            nums.len() as i32
        }

        let mut k = 2;

        for i in 2..nums.len() {
            if k < 2 || nums[index] > nums[k - 2] {
                nums[k] = nums[index];
                k += 1;
            }
        }

        k
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_remove_duplicates() {
        assert!(Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]) == 5);
    }
}
