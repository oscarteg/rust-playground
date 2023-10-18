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

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        //max(0) -> if larger then 0
        prices.windows(2).map(|i| (i[1] - i[0]).max(0)).sum()
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]),
            7
        );
    }
}
