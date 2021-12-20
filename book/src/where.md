# Where clauses

`where` is used to add constraints to generic types.

```rust
pub fn example<A,B>(x: A, y: B) -> bool
  where A: Ord + Eq + PartialEq + PartialOrd
      , B: Iterator
{
  //implementation
}
```
