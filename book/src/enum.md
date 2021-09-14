## Enum

As apposed to other programming languages, you can store values in Enums like follows;

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// This is the same as writing multiple structs

struct Quitmessage;

struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);
```

## Useful enums

Two particularly useful enums are `Result` and `Option`.

- `Result` gives you a way to represent whether an operation is successful or not, as well as a way to access the data or error of the... result. 

- `Option` gives you a way to represent whether something exists or not. This is generally used as a replacement for nullable types (which Rust does not have[*](https://doc.rust-lang.org/core/ptr/index.html)).

### When should you use which? 
A quick shortcut to remember which to use is _Option (to be, or not to be)_ and _Results (is everything ok?)_. Also Result is like an `Err`. 

