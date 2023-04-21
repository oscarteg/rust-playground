use std::{error::Error, str::FromStr};

#[derive(Debug)]
enum RomanChar {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl From<char> for RomanChar {
    fn from(s: char) -> RomanChar {
        match s {
            'I' => RomanChar::I,
            'V' => RomanChar::V,
            'X' => RomanChar::X,
            'L' => RomanChar::L,
            'C' => RomanChar::C,
            'D' => RomanChar::D,
            'M' => RomanChar::M,
            _ => panic!("Invalid roman numeral"),
        }
    }
}

impl RomanChar {
    fn to_int(&self, prev: i32) -> i32 {
        match *self {
            RomanChar::I if prev >= 5 => -1,
            RomanChar::I => 1,
            RomanChar::V if prev >= 10 => -5,
            RomanChar::V => 5,
            RomanChar::X if prev >= 50 => -10,
            RomanChar::X => 10,
            RomanChar::L if prev >= 100 => -50,
            RomanChar::L => 50,
            RomanChar::C if prev >= 500 => -100,
            RomanChar::C => 100,
            RomanChar::D if prev >= 1000 => -500,
            RomanChar::D => 500,
            RomanChar::M => 1000,
        }
    }
}

fn roman_to_int(s: String) -> i32 {
    s.chars().rfold(0, |acc, c| {
        let value = RomanChar::from(c).to_int(acc);
        acc + value
    })
}

#[test]
fn test() {
    assert_eq!(roman_to_int(String::from("I")), 1);
    assert_eq!(roman_to_int(String::from("IV")), 4);
    assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
}
