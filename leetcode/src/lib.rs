pub mod add_two_numbers;
pub mod binary_search;
pub mod datastructures_1;
pub mod first_bad_version;
pub mod heap_sort;
pub mod insertion_sort;
pub mod move_zeroes;
pub mod pattern_defeating_quicksort;
pub mod quicksort;
pub mod rotate_array;
pub mod search_insert_position;
pub mod sorted_squares;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
