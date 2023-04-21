struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

/// .
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

#[cfg(test)]

mod tests {
    use crate::iterator::fibonacci;

    #[test]
    fn test() {
        // `0..3` is an `Iterator` that generates: 0, 1, and 2.
        let mut sequence = 0..3;

        println!("Four consecutive `next` calls on 0..3");
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());

        println!("Iterate through 0..3 using `for`");
        for i in 0..3 {
            println!("> {}", i);
        }

        println!("The first four terms of the Fibonacci sequence are: ");
        for i in fibonacci().take(4) {
            println!("> {}", i);
        }

        println!("The next four terms of the Fibonacci sequence are: ");
        for i in fibonacci().skip(4).take(4) {
            println!("> {}", i);
        }

        let array = [1u32, 3, 3, 7];

        println!("Iterate the following array {:?}", &array);
        for i in array.iter() {
            println!("> {}", i);
        }
    }
}
