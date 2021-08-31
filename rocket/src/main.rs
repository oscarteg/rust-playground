use rocket::{get, routes};

#[get("/world")]
fn index() -> &'static str {
    "hello world"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/hello", routes![index])
        .launch()
        .await;
}
