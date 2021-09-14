# Structs

Structs are different from classes. 
- Structs are  _value_ types and classes are _reference_ types. Structs are because of that allocated on the stack or inline in containing types. While classes are allocated on the heap. __Rust has no classes.__
- Structs have no default constructor function. You can just create it yourself if you really want

```rust
struct Foo {
    bar: String
}

impl Foo {
    fn new() -> Self {
        Self {
            bar: String::from("Hello World")
        }
    }
}

fn main() {
    let foo = Foo::new();

    println!("{}", foo.bar.as_str())
}
```