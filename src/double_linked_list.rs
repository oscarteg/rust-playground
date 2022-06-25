// https://rust-unofficial.github.io/too-many-lists/

// List a = Empty Elem a (List a)

// Recursive type infinite size
// Solve with Box

/*
 * [] = stack
 () = Heap
 [ Elem, A, ptr] -> (Elem B, ptr) -> (Empty, *junk*) // One is not a node, one is not heap
 allocated


 [ptr] -> (Elem, A, ptr) -> (Elem B, *null*)

*/

#[derive(Debug)]
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}

pub struct Node {
    value: u64,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}
