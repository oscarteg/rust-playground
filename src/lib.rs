mod drop;
mod iterator;
mod lifetime;
mod macros;
mod patterns;
mod smart_pointers;
mod traits;

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[cfg(test)]
mod tests {
    use super::List::{Cons, Nil};
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    fn result() -> Result<(), String> {
        if (2 + 2).eq(&4) {
            Ok(())
        } else {
            Err(String::from("does not equal"))
        }
    }

    #[test]
    fn test_cons() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::from(Nil))))));
    }
}
