use std::str::FromStr;

use crate::Solution;

enum Roman {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl FromStr for Roman {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(Roman::I),
            "V" => Ok(Roman::V),
            "X" => Ok(Roman::X),
            "L" => Ok(Roman::L),
            "C" => Ok(Roman::C),
            "D" => Ok(Roman::D),
            "M" => Ok(Roman::M),
            _ => Err(()),
        }
    }
}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split(" ").last().unwrap_or("").len() as i32
    }

    pub fn roman_to_int(s: String) -> i32 {
        s.chars().into_iter().enumerate().fold(0, |acc, index| {
            let roman = Roman::from_str(&s.as_str()).unwrap();

            acc + roman as i32
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Solution;

    #[test]
    fn it_works() {
        let str = String::from("hello  world test");
        assert_eq!(Solution::length_of_last_word(str), 4)
    }
}
