use crate::ListNode;

/// Add two numbers from a linked list
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1_current = l1;
    let mut l2_current = l2;

    while l1_current.is_some() && l2_current.is_some() {}
    let list = ListNode::new(12);

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
