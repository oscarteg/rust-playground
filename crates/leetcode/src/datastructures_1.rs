#[cfg(test)]
mod tests {
    use std;
    use std::collections::HashSet;

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() > nums.iter().collect::<HashSet<_>>().len()
    }

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let mut max = 0;
        for &num in &nums {
            max = std::cmp::max(max + num, num);
            ans = std::cmp::max(ans, max);
        }
        ans
    }

    /// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
    /// Notice that the solution set must not contain duplicate triplets.
    pub fn three_sums(nums: Vec<i32>) -> Vec<Vec<i32>> {
        vec![vec![]]
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

    #[test]
    fn max_sub_array_test() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

        assert_eq!(max_sub_array(nums), 6);
    }
}
