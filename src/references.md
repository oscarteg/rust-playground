# References
When a function uses a value the ownership of that variable changes. A solution to this is as follows:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

It is tedious to always return the value again (and quite expensive). An other solution is the use of references.

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

This lets you refer to some value without taking ownership. Because it does not own it, ***the value it points too will not be dropped when the reference goes out of scope.***

Using references is like borrowing the ownership of that variable so that it doesn't get dropped when it gets out of scope. And, j***ust as variables are immutable by default, so are references.*** There is a short coming because references are immutable and so we can't change the value.

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

This code gives the compile error: `cannot borrow some_string as mutable, as it is behind a & reference`. We can fix this error easy by adding `mut` to the variable, to the reference and to accept a `mut` as parameter to the function.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

You can have only one mutable reference to a particular piece of data in a particular scope

### Dangling references

> A pointer that references a location in memory that may have been given to someone else.

An example:

```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

- At any given time, you can have *either* one mutable reference *or* any number of immutable references.
- References must always be valid.