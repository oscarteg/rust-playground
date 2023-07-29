// Call the buildMaxHeap() function on the list. Also referred to as heapify(), this builds a heap from a list in {\displaystyle O(n)}O(n) operations.
// Swap the first element of the list with the final element. Decrease the considered range of the list by one.
// Call the siftDown() function on the list to sift the new first element to its appropriate index in the heap.
// Go to step (2) unless the considered range of the list is one element.

///
/// procedure heapsort(a, count) is
//     input: an unordered array a of length count
//
//     (Build the heap in array a so that largest value is at the root)
//     heapify(a, count)
//
//     (The following loop maintains the invariants that a[0:end] is a heap and every element
//      beyond end is greater than everything before it (so a[end:count] is in sorted order))
//     end ← count - 1
//     while end > 0 do
//         (a[0] is the root and largest value. The swap moves it in front of the sorted elements.)
//         swap(a[end], a[0])
//         (the heap size is reduced by one)
//         end ← end - 1
//         (the swap ruined the heap property, so restore it)
//         siftDown(a, 0, end)

#[derive(Debug)]
struct Node<T> {
    // The Box is one of the smart pointers. It saves an address that points to a data in memory. Boxhelps us to create a BinaryTree struct with an unknown size, so that we can grow the Binary Tree by inserting nodes without thinking ahead how many nodes we'll have when creating the tree.
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
    pub fn left(self, node: Node<T>) -> Option<Box<Node<T>>> {
        self.left
    }

    pub fn right(self, node: Node<T>) -> Option<Box<Node<T>>> {
        self.right
    }
}

pub fn heap_sort(nums: &mut [i32], k: i32) {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
