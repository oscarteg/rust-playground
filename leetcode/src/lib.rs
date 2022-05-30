mod add_two_numbers;
mod binary_search;
mod first_bad_version;
mod heap_sort;
mod insertion_sort;
mod pattern_defeating_quicksort;
mod quicksort;
mod rotate_array;
mod search_insert_position;
mod sorted_squares;

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
