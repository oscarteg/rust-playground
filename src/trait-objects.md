# Trait (objects)

Trait object for defining common behaviour. It is *like* polymorphism in OO.

trait objects are more like objects in other languages in the sense that they combine data and behavior

You can't add data to trait object

What is dyn

Rust compiler needs to know how much space every function's return type required. You HAVE to return a concrete type. You can't return a Trait. Or can you?

You can return a Box. A Box is a reference to something on the heap. The reference is known at compile time so you can return a box without knowing what it references to.

The two major cases for `Box<T>` are:

- recursive types, like a list `enum List<T> { Nil, Cons(T, Box<List<T>>) }`, to avoid them being infinite size (this is what libsyntax’s `P` is for)
- owned trait objects

There are some more niche cases, such as:

- atomic/lock-free operations where CPUs generally only support atomic operations on things that are pointer sized (or less),
- for performance if `T` is large and is being moved around a lot, using a `Box<T>` instead will avoid doing big `memcpy`s

The first one is generally handled by low-level libraries such as crossbeam (i.e. the APIs of such libraries will make the decisions `Box`ing or not, and you don’t have to think, just follow their lead), and the second is reasonably rare: the cost of dynamic allocation and the (typically) poorer cache locality will mean `Box<T>` is usually slower than just `T` itself.