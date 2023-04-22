mod add;
mod double_linked_list;
mod higher_rank_trait_bounds;
mod iterator;
mod largest_product;
mod rust_is_hard;
mod serde;
mod traits;
mod web_scraping;

use rand::Rng;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Read};
use std::net::IpAddr;
use std::panic::panic_any;

fn main() {
    let f = File::open("./hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            other_error => {
                panic!("Problem opening file {:?}", other_error)
            }
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|error| panic!("Problem creating file {:?}", error))
        } else {
            panic!("Problem opening file {:?}", error)
        }
    });

    println!("{:?}", read_username_from_file_v2().unwrap());
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    // guess_the_number();

    println!("{}", largest(&vec![1, 2, 3]));
}

fn read_username_from_file_v2() -> Result<String, Error> {
    // Version 2 using ? syntax
    // let mut f = File::open("hello.txt")?;

    // let mut s = String::new();
    //
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // Version with chaining
    // let mut s = String::new();
    //
    // File::open("hello.txt")?.read_to_string(& mut s)?;
    // Ok(s)
    fs::read_to_string("hello.txt")
}

fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn guess_the_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // --snip--

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        // if guess < 1 || guess > 100 {
        //     println!("The secret number will be between 1 and 100.");
        //     continue;
        // }

        match guess.value.cmp(&secret_number) {
            // --snip--
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn largest<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];
    for item in list {
        if item > &*largest {
            largest = &item;
        }
    }
    largest
}

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("must be between 1 - 100 {}", value);
        }

        Guess { value }
    }

    pub fn value(&mut self) -> i32 {
        self.value
    }
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Pair<T>
where
    T: Display + PartialOrd,
{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
