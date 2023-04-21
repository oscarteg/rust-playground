use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Pagination {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Users {
    users: Vec<Users>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
