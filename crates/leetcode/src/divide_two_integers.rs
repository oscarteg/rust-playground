use crate::Solution;
impl Solution {
    /// Given two integers dividend and divisor, divide two integers without using multiplication, division, and mod operator.
    //
    // The integer division should truncate toward zero, which means losing its fractional part. For example, 8.345 would be truncated to 8, and -2.7335 would be truncated to -2.
    //
    // Return the quotient after dividing dividend by divisor.
    //
    // Note: Assume we are dealing with an environment that could only store integers within the 32-bit signed integer range: [−231, 231 − 1]. For this problem, if the quotient is strictly greater than 231 - 1, then return 231 - 1, and if the quotient is strictly less than -231, then return -231.
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        // Handle edge cases
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        if divisor == 1 {
            return dividend;
        }
        if divisor == -1 {
            return -dividend;
        }

        // Determine sign of the result
        // We use the booleans as number property
        // XOR 1 ^ 1 = 0 so the sign is negative
        // XOR 1 ^ 0 = 1 so the sign is negative
        let sign = (dividend < 0) ^ (divisor < 0);

        // Convert to positive numbers (be careful with overflow!)
        let mut dividend = dividend.abs() as i64;
        let divisor = divisor.abs() as i64;

        // Perform division by repeated subtraction
        let mut quotient = 0;
        let mut temp = 0;

        // while dividend >= divisor {
        //     dividend -= divisor;
        //     quotient += 1;
        // }

        // Perform division using bit manipulation
        // Generated, I have no idea how to do this
        for i in (0..32).rev() {
            println!("i: {}", i);
            if (temp + (divisor << i)) <= dividend {
                temp += divisor << i;
                println!("temp: {}", temp);
                quotient |= 1 << i;
            }
        }

        if sign {
            -quotient
        } else {
            quotient
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_divide() {
        assert_eq!(Solution::divide(10, 3), 3);
    }

    #[test]
    fn test_divide_negative() {
        assert_eq!(Solution::divide(7, -3), -2);
    }

    #[test]
    fn test_divide_negative_negative() {
        assert_eq!(Solution::divide(-3, -3), 1);
    }
}