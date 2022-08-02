// https://www.sheshbabu.com/posts/rust-error-handling/

# Error handling

## Ignore the error

Using `unwrap()` we tell the compiler we are sure that there is a value and to get the contents of the Ok()

```rust
use std::fs;

fn main() {
    let content = fs::read_to_string("./Cargo.toml").unwrap();
    println("{}, content");
}

```

## Terminate the program

Using `expect()` we terminate the program with a message when we can't get the content from the Option

```rust
use std::fs;

fn main() {
    let content = fs::read_to_string("./Cargo.toml").expect("Can't read file");
    println("{}, content");
}

```

## Use a fallback value

```rust
use std::env;

fn main() {
  let port = env::var("PORT").unwrap_or("3000".to_string());
  println!("{}", port);
}
```

## Bubble up the error

```rust
use std::collections::HashMap;

fn main() {
  match get_current_date() {
    Ok(date) => println!("We've time travelled to {}!!", date),
    Err(e) => eprintln!("Oh noes, we don't know which era we're in! :( \n  {}", e),
  }
}

fn get_current_date() -> Result<String, reqwest::Error> {
  let url = "https://postman-echo.com/time/object";

  // Bubbling up the error with the ? shorthandle
  let res = reqwest::blocking::get(url)?.json::<HashMap<String, i32>>()?;
  let date = res["years"].to_string();

  /**
  // Bubbling up an error the manual way
    let result = reqwest::blocking::get(url);

    let response = match result {
      Ok(res) => res,
      Err(err) => return Err(err),
    };

    let body = response.json::<HashMap<String, i32>>();

    let json = match body {
      Ok(json) => json,
      Err(err) => return Err(err),
    };

    let date = json["years"].to_string();
  **/

  Ok(date)
}

```

## Bubble up multiple errors

Returning a trait object `Box<dyn std::error::Error>` is very convenient when we want to return multiple errors!

```rust

  use chrono::NaiveDate;
  use std::collections::HashMap;

  fn main() {
    match get_current_date() {
      Ok(date) => println!("We've time travelled to {}!!", date),
      Err(e) => eprintln!("Oh noes, we don't know which era we're in! :( \n  {}", e),
    }
  }

  fn get_current_date() -> Result<String, reqwest::Error> {
    let url = "https://postman-echo.com/time/object";
    let res = reqwest::blocking::get(url)?.json::<HashMap<String, i32>>()?;
    let formatted_date = format!("{}-{}-{}", res["years"], res["months"] + 1, res["date"]);

    // Compile error because we return a different error here than the return type indicates
    // The return types needs to be: Result<String, Box<dyn std::error::Error>>
    let parsed_date = NaiveDate::parse_from_str(formatted_date.as_str(), "%Y-%m-%d")?;
    let date = parsed_date.format("%Y %B %d").to_string();

    Ok(date)
  }

```

You can also use the crate: [Anyhow](https://github.com/dtolnay/anyhow).

Example

```rust

```

## Match boxed errors

## Libraries vs Applications

## Create custom errors

## Bubble up custom errors

## Match custom errors
