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

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let map: HashMap<i32, usize> = HashMap::new();
        let max: Option<i32> = nums
            .into_iter()
            .fold(map, |mut m, i| {
                *m.entry(i).or_default() += 1;
                m
            })
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k);

        max.unwrap_or(0)
    }
}

impl Solution {
    // Three different solutions
    // First: rust power lol
    pub fn rotate_1(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len();
        // Modulo to not rotate unnecessary
        nums.rotate_right((k as usize) % length);
    }

    // Reverse
    pub fn rotate_2(nums: &mut Vec<i32>, k: i32) {
        let k = (k as usize) % nums.len();

        Solution::reverse(nums);

        Solution::reverse(&mut nums[0..k]);
        Solution::reverse(&mut nums[k..]);
    }

    // O(1)
    pub fn rotate_3(nums: &mut Vec<i32>, k: i32) {}

    pub fn reverse(nums: &mut [i32]) {
        let mid = nums.len() / 2;
        for i in 0..mid {
            // let opp = nums.len() - i - 1;
            //
            // nums.swap(i, opp);

            todo!()
        }
    }
}

impl Solution {
    // This goes by the notion that we check if the character is duplicate and counts the position between those characters
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_length = 0;
        // Could also use a hashmap
        let mut position_map = [0; 128];
        let mut start = 0;

        s.chars().enumerate().for_each(|(end, ch)| {
            // Move start the latest unknown char
            start = start.max(position_map[ch as usize]);

            // length is distance between start / end
            max_length = max_length.max(end - start + 1);

            // Last occurrence of ch set in map
            position_map[ch as usize] = end + 1;
        });

        max_length as i32
    }
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((0, i32::MAX), |(max_profit, cost), &price| {
                let new_cost = i32::min(cost, price);
                let new_profit = i32::max(max_profit, price - cost);
                (new_profit, new_cost)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn max_profit() {
        let z = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(z), 5);
    }

    /// Generated by ChatGPT
    #[test]
    fn test_length_of_longest_substring() {
        // Test with an empty string
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);

        // Test with a string without repeating characters
        assert_eq!(
            Solution::length_of_longest_substring("abcdefg".to_string()),
            7
        );

        // Test with a string having repeating characters
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );

        // Test with a string containing only one character
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1);

        // Test with a string containing mixed characters with repetitions
        assert_eq!(
            Solution::length_of_longest_substring("abcaebcd".to_string()),
            5
        );
    }

    #[test]
    fn test_rotate() {
        let mut z = vec![1, 2, 3, 3, 3];
        Solution::rotate_1(&mut z, 2);

        assert_eq!(z, vec![3, 3, 1, 2, 3]);

        let mut x = vec![1, 2, 3, 4, 5, 6];
        Solution::rotate_2(&mut x, 5);

        assert_eq!(x, vec![2, 3, 4, 5, 6, 1]);
    }

    #[test]
    fn test_majority_element() {
        let z = vec![1, 2, 3, 3, 3];

        assert_eq!(Solution::majority_element(z), 3);
    }

    #[test]
    fn test_merge() {
        let mut z = vec![1, 2, 3, 0, 0, 0];
        Solution::merge(&mut z, 3, &mut vec![2, 5, 6], 3);

        println!("{:?}", z);

        assert!(z == vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_find_duplicate() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
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
