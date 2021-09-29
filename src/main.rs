#[macro_use] extern crate rocket;

mod router;
use router::routes::foo;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![foo])
        // .mount("/", routes![delay])
}