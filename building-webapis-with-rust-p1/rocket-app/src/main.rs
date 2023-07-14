// add crate-level allow attribute (notice the !)
#![allow(dead_code)]
/// instructing the compiler that we're gonna make use of rocket macros
#[macro_use] extern crate rocket;
// importing/making use of json
use rocket::{serde::json::{Value, json}, response::status, http::{hyper::Request, Status}, data::Outcome, request::FromRequest};

/// CRUD endpoints
/// GET list existing
/// GET show single
/// POST create new
/// PUT update existing
/// DELETE delete existing
/// 
/// struct for username and password
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

/// implementing factory methods for the struct
impl BasicAuth {
    // factory function gets passed the value of the header
    // which is Basic "QWxhZGRpbjpvcGVuIHNlc2FtZQ=="
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        // split header value by whitespace and collect it into a vector
        let split = header.split_whitespace().collect::<Vec<_>>();

        if split.len() != 2 {
            return None;
        }

        if split[0] != "Basic" {
            return None;
        }
    
        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base_64_string: &str) -> Option<BasicAuth> {
        // trying to convert result to option
        // ? tells Rust only to continue to next line if ok
        let decoded = base64::decode(base_64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();

        // if exactly username and password pair are present
        if split.len() != 2 {
            return None;
        }
        
        let (username, password) = (split[0].to_string(), split[1].to_string());

        // let username = "foo".to_string();
        // let password = "bar".to_string();
        
        Some(BasicAuth { username, password })

    }
}

/// implementing request guard
/// https://rocket.rs/v0.4/guide/requests/#request-guards
/// from the documentation: a request guard protects a handler from being called 
/// erroneously based on information contained in an incoming request. More 
/// specifically, a request guard is a type that represents an arbitrary validation 
/// policy. The validation policy is implemented through the FromRequest trait. 
/// Every type that implements FromRequest is a request guard.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(auth)
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }

}

/// CRUD endpoints for fictional rustaceans database
/// curl http://127.0.0.1:8000/rustaceans
/// making this route only accessible if someone specifies 
/// Basic access authentication with header 
/// see https://en.wikipedia.org/wiki/Basic_access_authentication
/// Base64 encoding of Aladdin:open sesame
/// curl http://127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
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
