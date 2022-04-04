pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low: i32 = 0;
    let mut high: i32 = nums.len() as i32 - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        let val = nums[mid as usize];

        if val == target {
            return mid;
        }

        if val > target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::binary_search::search;

    #[test]
    fn it_works() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4)
    }

    #[test]
    fn returns_when_not_found() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1)
    }
}
