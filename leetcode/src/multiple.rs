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

#[cfg(test)]
mod tests {
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
