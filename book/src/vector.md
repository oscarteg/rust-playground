# Vector

A _vector_ is not an iterator.

But it implements the trait [IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) `IntoIterator`, which the `for` loop uses to convert the vector into the required iterator.

In the <a href="https://doc.rust-lang.org/std/vec/struct.Vec.html">documentation for `Vec`</a> you can see that `IntoIterator` is implemented in three ways:

- for `Vec<T>`, which is moved and the iterator returns items of type `T`,
- for a shared reference `&Vec<T>`, where the iterator returns shared references `&T`,
- and for `&mut Vec<T>`, where mutable references are returned.

<a href="https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter">`iter()`</a> is just a method in `Vec` to convert `Vec<T>` directly into an iterator that returns shared references, without first converting it into a reference. There is a sibling method <a href="https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter_mut">`iter_mut()`</a> for producing mutable references.