/// instructing the compiler that we're gonna make use of rocket macros
#[macro_use] extern crate rocket;
// importing/making use of json
use rocket::serde::json::{Value, json};

/// defining endpoint with attribute
/// incoming get request to "/"
#[get("/")]
// returning Value from serde::json
fn hello() -> Value {
    // making use of json makro
    json!("Hello, world!")
}

/// main function will build rocket framework
/// after launch it will start listening to the requests
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![hello])
        .launch()
        .await;
}
