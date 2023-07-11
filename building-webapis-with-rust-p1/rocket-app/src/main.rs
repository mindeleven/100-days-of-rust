/// instructing the compiler that we're gonna make use of rocket macros
#[macro_use] extern crate rocket;

/// defining endpoint with attribute
/// incoming get request to "/"
#[get("/")]
fn hello() -> &'static str {
    "Hello, world!\n"
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
