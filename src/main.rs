#[macro_use] extern crate rocket;

mod routes;
mod controllers;

use routes::product_routes::get_products_route;

#[launch]
fn init() -> _ {
    rocket::build()
        .mount("/api/v1", routes![get_products_route])
}