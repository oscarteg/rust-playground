use crate::ListNode;

/// Add two numbers from a linked list
pub fn add_two_numbers(
    left: Option<Box<ListNode>>,
    right: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // let dummy_head = ListNode::new(0);
    // let _curr = &dummy_head;
    // let mut carry = 0;

    None
}

#[cfg(test)]
mod tests {
    use crate::ListNode;

    use super::add_two_numbers;

    #[test]
    fn it_works() {
        let l1: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let l2: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let l3: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode { val: 6, next: None })),
            })),
        }));

        assert_eq!(add_two_numbers(l1, l2), l3);
    }
}
