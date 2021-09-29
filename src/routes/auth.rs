
#[get("/login")]
pub fn login() -> &'static str {
    "Login!"
}

#[get("/logout")]
pub fn logout() -> &'static str {
    "Log out!"
}