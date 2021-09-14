# Ownership

Memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None fo the ownership features slow down your program while running.

There are 2 types of memory. **Stack** stores values in the order it gets them and removes in the opposite order *"last in, first out".* You can push items on or pop items of the stack. Data stored on the stack has to have a fixed size. The heap is a random place where the compiler finds a random spot and returns a pointer, this is called *allocating* memory.

A couple of notes:

- Each value in Rust has a variable that's called its *owner.*
- There can be only **ONE** owner.
- When the owner goes out of scope, the value will be dropped.

A scope is the range within a program for which an item is valid.

In Rust memory is automatically returned once the variable that owns it goes out of scope.

```rust
let s3 = String::from("Testing");
let s4 = s3;

println!("{}", s3);
```

 

## Ownership, stack and the heap

