
#[get("/")]
pub fn foo() -> &'static str {
    "Hello, world!"
}