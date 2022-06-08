#[cfg(test)]
mod tests {

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zeros = Vec::new();
        // only retain the
        nums.retain(|elem| {
            if *elem != 0 {
                true
            } else {
                zeros.push(0);
                false
            }
        });

        nums.append(&mut zeros);
    }

    pub fn move_zeroes_in_place(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;

        while i < nums.len() {
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
            i += 1;
        }
    }

    #[test]
    fn it_works() {
        let mut array = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut array);
        assert_eq!(&array, &vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn it_works_optimal() {
        let mut array = vec![0, 1, 0, 3, 12];
        move_zeroes_in_place(&mut array);
        assert_eq!(&array, &vec![1, 3, 12, 0, 0]);
    }
}
