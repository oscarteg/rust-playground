mod lifetime;
mod traits;

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
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
}
