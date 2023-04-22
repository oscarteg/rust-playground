use std::fmt::Display;

#[test]
fn test_lifetime() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
        println!("{}", result)
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn test() {
    let novel = String::from("call me ");
    let first_sentence = novel.split('.').next().expect("could not find a .");

    let _i = ImportantExcerpt {
        part: first_sentence,
    };
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

#[test]
fn test_announcement() {
    let x = String::from("testing");
    let y = String::from("123");
    let ann = String::from("hello");
    longest_with_an_announcement(&x, &y, ann);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Account, {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
