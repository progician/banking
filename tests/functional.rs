use assert2::assert;
use rstest::{fixture, rstest};
use rocket::local::blocking::Client;
use rocket::http::Status;

#[path = "../src/main.rs"]
mod main;

use main::rocket;


#[fixture]
fn client() -> Client {
    Client::tracked(rocket()).expect("valid rocket instance")
}

#[rstest]
fn newly_created_accounts_are_empty(client: Client) {
    let response = client.get("/").dispatch();
    assert!(response.status() == Status::Ok);
    assert!(response.into_string().unwrap() == "Hello, World!");
}
