use crate::Solution;

impl Solution {

    /// We need to generate strings of length 2n (n open parentheses and n closing parentheses).
    // At any point, we can add an open parenthesis if we haven't used all n.
    // We can add a closing parenthesis as long as it doesn't exceed the number of open parentheses.
    // We'll use recursion to build our combinations.
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        Self::backtrack(n, n, String::from(""), &mut result);
        result
    }

    fn backtrack(open: i32, close: i32, current: String, result: &mut Vec<String>) {

        println!("open: {}, close: {}, current: {}, result: {:?}", open, close, current, result);
        if open == 0 && close == 0 {
            result.push(current);
            return;
        }

        if open > 0 {
            Self::backtrack(open - 1, close, current.clone()+ "(", result);
        }


        if close > open {
            Self::backtrack(open, close - 1, current + ")", result);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_generate_parenthesis() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
