use std::ops::Add;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {} {}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify<T>(item: &T) -> &dyn Summary
where
    T: Summary,
{
    item.summarize();

    item
}

#[derive(Debug, PartialEq)]
struct Counter {
    value: i32,
}

impl Add for Counter {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { value: rhs.value }
    }
}

#[test]
fn test_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(" of course, as you probably"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    notify(&tweet);
    notify(&article);
}
