use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Pagination {
    limit: u64,
    offset: u64,
    total: u64,
}

#[derive(Debug)]
struct Users {
    users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

trait Pagination {
    fn next_page(&self);
    fn previous_page(&self);
}
