use lazy_static;
use rocket::http::{ContentType, Status};
use shopit_be_rust::data::db::ResponseUser;
use serde_json;

mod common;

#[test]
fn ping_test() {
    let client = common::setup();
    let mut response = client.get("/ping").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("PONG!".into()));
}

#[test]
fn user_list_rt_test() {
    let client = common::setup();
    let mut response = client.get("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let mut response_body = response.body_string().unwrap();
    response_body.retain(|c| !c.is_numeric());
    assert_eq!(response_body.as_str(), "[]")
}

#[test]
fn new_user_rt_test() {
    let client = common::setup();
    let mut response = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "John Doe",
            "email": "j.doe@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.body_string().expect("Response body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "j.doe@m.com");
}

#[test]
fn info_user_rt_test() {
    let client = common::setup();
    let mut response = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "John Doe",
            "email": "j.doe@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.body_string().expect("Response body");
    let user_new: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    let id = user_new.id;
    let mut response = client.get(format!("/api/users/{}", id)).dispatch();
    let response_body = response.body_string().expect("Response body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "j.doe@m.com");
    assert_eq!(user.id, id);
}

#[test]
fn update_user_rt_test() {
    // let client = common::setup();
    // let mut response = client.put("/api/users/1").dispatch();
    // assert_eq!(response.status(), Status::Ok);
    // assert_eq!(response.content_type(), Some(ContentType::JSON));
    // assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Update info for user 1\"}".into()));
}

#[test]
fn delete_user_rt_test() {
    // let client = common::setup();
    // let mut response = client.delete("/api/users/1").dispatch();
    // assert_eq!(response.status(), Status::Ok);
    // assert_eq!(response.content_type(), Some(ContentType::JSON));
    // assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Delete user 1\"}".into()));
}