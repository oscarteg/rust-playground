struct Solution {
    is_bad_version: i32,
}

impl Solution {
    fn isBadVersion(&self, version: i32) -> bool {
        self.is_bad_version == version
    }
    
    

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut low: i32 = 0;
        let mut high: i32 = n;

        while high > (low + 1) {
            let mid = low + (high - low) / 3;

            if self.isBadVersion(mid) {
                high = mid;
            } else {
                low = mid;
            }
        }
        high
    }
}

#[cfg(test)]
mod tests {
    use crate::first_bad_version::Solution;

    #[test]
    fn it_works() {
        let sol = Solution { is_bad_version: 4 };
        assert_eq!(sol.first_bad_version(5), 4);
    }

    #[test]
    fn returns_when_not_found() {
        let sol = Solution { is_bad_version: 1 };
        assert_eq!(sol.first_bad_version(1), 1);
    }
}
