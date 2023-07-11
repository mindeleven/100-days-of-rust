// add crate-level allow attribute (notice the !)
#![allow(dead_code)]
/// instructing the compiler that we're gonna make use of rocket macros
#[macro_use] extern crate rocket;
// importing/making use of json
use rocket::{serde::json::{Value, json}, response::status};

/// CRUD endpoints
/// GET list existing
/// GET show single
/// POST create new
/// PUT update existing
/// DELETE delete existing
/// 
/// CRUD endpoints for fictional rustaceans database
#[get("/rustaceans")]
fn get_rustaceans() -> Value {
    json!([
        {"id": 3, "name": "John Doe", "email": "john@doe.com"},
        {"id": 4, "name": "John Again Doe", "email": "john.again@doe.com"}
    ])
}
#[get("/rustaceans/<id>")]
fn view_rustaceans(id: i32) -> Value {
    json!({"id": 3, "name": "John Doe", "email": "john@doe.com"})
}
#[post("/rustaceans", format="json")]
fn create_rustaceans() -> Value {
    json!({"id": 3, "name": "John Doe", "email": "john@doe.com"})
}
#[put("/rustaceans/<id>", format="json")]
fn update_rustaceans(id: i32) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[delete("/rustaceans/<id>")]
fn delete_rustaceans(id: i32) -> status::NoContent {
    status::NoContent
}

/// 
/// defining endpoint with attribute
/// incoming get request to "/"
#[get("/")]
/// returning Value from serde::json
fn hello() -> Value {
    // making use of json makro
    json!("Hello, world!")
}

/// 

/// main function will build rocket framework
/// after launch it will start listening to the requests
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![hello])
        .launch()
        .await;
}
