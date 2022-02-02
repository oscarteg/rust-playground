# Iterators

Iterator is a trait that describes 



There are three common methods which can create iterators from a collection:

iter(), which iterates over &T. So by reference
iter_mut(), which iterates over &mut T. So by mut reference
into_iter(), which iterates over T. Iterates and _moves_ into the new scope. 

In short: _If you just need to "look at" the data, use `iter`, if you need to edit/mutate it, use `iter_mut`, and if you need to give it a new owner, use `into_iter`. _

The iterator returned by into_iter may yield any of T, &T or &mut T, depending on the context.
The iterator returned by iter will yield &T, by convention.
The iterator returned by iter_mut will yield &mut T, by convention.



```rust
pub trait IntoIterator 
where
    <Self::IntoIter as Iterator>::Item == Self::Item, 
{
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}

// Vec implements IntoIterator 3 times
impl<T> IntoIterator for Vec<T>
impl<'a, T> IntoIterator for &'a Vec<T>
impl<'a, T> IntoIterator for &'a mut Vec<T>
```

You implement this trait when you want to specify how a particular type is to be converted into an iterator. Most notably, if a type implements `IntoIterator` it can be used in a `for` loop.



