use std::collections::HashMap;

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
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, cur| {
            if acc & (1 << cur) > 0 {
                return cur;
            }
            acc | (1 << cur)
        })
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
        for i in &nums {
            if left_sum == total_sum - left_sum - nums[*i as usize] {
                return *i as i32;
            }
            left_sum += nums[*i as usize]
        }

        -1
    }
}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map_s = HashMap::new();
        let mut map_t = HashMap::new();

        for (i, (s_char, t_char)) in s.chars().zip(t.chars()).enumerate() {
            if map_s.insert(s_char, i) != map_t.insert(t_char, i) {
                return false;
            }
        }

        true
    }
}

impl Solution {
    pub fn merge(nums: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut j = m as usize - 1;
        let mut k = n as usize - 1;

        // Loop backwards in array, m+n is always the size of array
        for i in (0..(m + n) as usize).rev() {
            match nums.get(j).cmp(&nums2.get(k)) {
                std::cmp::Ordering::Less => {
                    nums[i] = nums2[k];
                    k -= 1;
                }
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    nums[k] = nums[j];
                    j -= 1;
                }
            }
        }
    }
}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test_merge() {
        let mut z = vec![1, 2, 3, 0, 0, 0];
        Solution::merge(&mut z, 3, &mut vec![2, 5, 6], 3);

        println!("{:?}", z);

        assert!(z == vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_find_duplicate() {
        assert!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]) == 2);
        assert!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]) == 3);
    }

    #[test]
    fn test_is_isomorphic() {
        assert!(Solution::is_isomorphic(
            String::from("egg"),
            String::from("add")
        ));
        assert!(!Solution::is_isomorphic(
            String::from("foo"),
            String::from("bar")
        ));

        assert!(Solution::is_isomorphic(
            String::from("paper"),
            String::from("title")
        ));
    }

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
