#[cfg(test)]
mod tests {
    use std;
    use std::collections::HashSet;

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() > nums.iter().collect::<HashSet<_>>().len()
    }

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // the answer
        let mut ans = i32::MIN;
        // the max number that can be achieved
        let mut max = 0;
        if nums.len() > 1 {
            // loop over all the number
            for (_, &v) in nums.iter().enumerate() {
                // the new max is
                max = std::cmp::max(max + v, v);
                ans = std::cmp::max(ans, max);
            }
            ans
        } else {
            nums[0]
        }
    }

    /// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
    /// Notice that the solution set must not contain duplicate triplets.
    pub fn three_sums(nums: Vec<i32>) -> Vec<Vec<i32>> {
        return vec![vec![]];
    }

    #[test]
    fn contains_dupe() {
        assert!(contains_duplicate(vec![1, 2, 3, 3]));
    }

    #[test]
    fn not_contains_dupe() {
        assert!(!contains_duplicate(vec![1, 2, 3]));
    }

    #[ignore]
    #[test]
    fn contains_three_sums() {
        let nums = vec![-1, 0, 1, 2, -1, -4];

        assert!(three_sums(nums) == vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }
}
