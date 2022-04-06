pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let t = &target;
    for (i, n) in nums.iter().enumerate() {
        if n >= t {
            return i as i32;
        }
    }
    nums.len() as i32
}

pub fn search_insert_v2(nums: Vec<i32>, target: i32) -> i32 {
    // Iterate over all the numbers
    //
    nums.iter().take_while(|x| **x < target).count() as i32
}

#[cfg(test)]
mod tests {
    use crate::search_insert_position::{search_insert, search_insert_v2};

    #[test]
    fn it_works() {
        assert_eq!(search_insert_v2(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn returns_index_where_it_should_be() {
        assert_eq!(search_insert_v2(vec![1, 3, 5, 6], 2), 1);
    }
}
