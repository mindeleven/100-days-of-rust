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
/// curl http://127.0.0.1:8000/rustaceans
#[get("/rustaceans")]
fn get_rustaceans() -> Value {
    json!([
        {"id": 3, "name": "John Doe", "email": "john@doe.com"},
        {"id": 4, "name": "John Again Doe", "email": "john.again@doe.com"}
    ])
}
/// curl http://127.0.0.1:8000/rustaceans/66           
#[get("/rustaceans/<id>")]
fn view_rustaceans(id: i32) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
/// curl http://127.0.0.1:8000/rustaceans -X POST -H 'Content-type: application/json'
#[post("/rustaceans", format="json")]
fn create_rustaceans() -> Value {
    json!({"id": 3, "name": "John Doe", "email": "john@doe.com"})
}
/// curl http://127.0.0.1:8000/rustaceans/1 -X PUT -H 'Content-type: application/json'
#[put("/rustaceans/<id>", format="json")]
fn update_rustaceans(id: i32) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
/// curl http://127.0.0.1:8000/rustaceans/123 -X DELETE
/// curl http://127.0.0.1:8000/rustaceans/123 -X DELETE -I 
#[delete("/rustaceans/<_id>")]
fn delete_rustaceans(_id: i32) -> status::NoContent {
    status::NoContent
}

/// Rocket comes with predefined error catches
/// which can be overwritten (and have to be reistered in main)
#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
}

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
        .mount("/", routes![
            hello,
            get_rustaceans,
            view_rustaceans,
            create_rustaceans,
            update_rustaceans,
            delete_rustaceans
        ])
        .register("/", catchers![
            not_found
        ])
        .launch()
        .await;
}
