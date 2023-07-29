use crate::Solution;

pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
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

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::binary_search;

    #[test]
    fn it_works() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 9), 4)
    }

    #[test]
    fn returns_when_not_found() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 2), -1)
    }
}
