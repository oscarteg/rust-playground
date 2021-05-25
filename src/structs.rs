#[derive(Eq, PartialEq)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// #[derive(Eq, PartialEq)]
// struct UserStr {
// username: &str,
// email: &str,
// sign_in_count: u64,
// active: bool,
// }


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 0,
        active: true,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// #[test]
// fn bar() {
//     let user1 = UserStr {
//         email: "example@example.com",
//         username: "example",
//         sign_in_count: 0,
//         active: true,
//     };
// }
//
#[test]
fn foo() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

#[test]
fn test_struct() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("oscar"),
        active: true,
        sign_in_count: 0,
    };

    user1.email = String::from("testing");
}

#[test]
fn test_build_user() {
    let user1 = build_user(String::from("example@example.com"), String::from("example"));
    let user2 = User {
        email: String::from("example@example.com"),
        username: String::from("example"),
        sign_in_count: 0,
        active: true,
    };
}

#[test]
fn structs() {
    let width = 30;
    let height = 50;

    println!("The area is {}", area(width, height));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

struct Rectangle {
    height: u32,
    width: u32,
}
