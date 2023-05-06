use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(input: Vec<String>) -> String {
        input
            .into_iter()
            .reduce(|acc, cur| {
                acc.chars()
                    .zip(cur.chars())
                    .take_while(|(a, c)| a == c)
                    .map(|(c, _)| c)
                    .collect()
            })
            .unwrap()
    }
}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        let v: Vec<i32> = Vec::new();
        nums.into_iter()
            .map(|v| {
                sum += v;
                sum
            })
            .collect::<Vec<i32>>()
    }
}

impl Solution {
    // Loop and check the remaining expected
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        // First loop to create total sum
        let total_sum: i32 = nums.iter().sum();

        let mut left_sum = 0;

        // loop until end of array
        for i in 0..nums.len() {
            if left_sum == total_sum - left_sum - nums[i] {
                return i as i32;
            }
            left_sum += nums[i]
        }

        -1
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn pivot_index() {
        assert_eq!(super::Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn test() {
        let input = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let output = "fl".to_string();
        assert_eq!(super::Solution::longest_common_prefix(input), output);

        let input = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let output = "".to_string();
        assert_eq!(super::Solution::longest_common_prefix(input), output);
    }
}
