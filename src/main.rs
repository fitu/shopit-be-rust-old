#[macro_use] extern crate rocket;

mod routes;
use routes::auth::login;
use routes::auth::logout;

#[launch]
fn init() -> _ {
    rocket::build()
        .mount("/", routes![login, logout])
}