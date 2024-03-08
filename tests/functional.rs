use assert2::assert;
use rstest::{fixture, rstest};
use rocket::local::blocking::Client;
use rocket::http::Status;
use rocket::serde::Deserialize;

#[path = "../src/main.rs"]
mod main;

use main::rocket;


struct TestClient {
    client: Client,
}

#[derive(Deserialize)]
struct AccountIDResponse {
    account_id: AccountID,
}


#[derive(Deserialize)]
struct BalanceResponse {
    account_id: AccountID,
    balance_amount: u64,
    balance_currency: String,
}


type AccountID = u64;


impl TestClient {
    fn new() -> Self {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        TestClient { client }
    }

    fn create_account(&self) -> AccountID {
        let response = self.client.get("/api/create").dispatch();
        assert!(response.status() == Status::Ok);
        response.into_json::<AccountIDResponse>().unwrap().account_id
    }

    fn balance_of(&self, account_id: AccountID) -> u64 {
        let response = self.client.get(format!("/api/balance/{}", account_id)).dispatch();
        assert!(response.status() == Status::Ok);
        response.into_json::<BalanceResponse>().unwrap().balance_amount
    }
}


#[fixture]
fn client() -> TestClient {
    TestClient::new()
}

#[rstest]
fn newly_created_accounts_are_empty(client: TestClient) {
    let account_id = client.create_account();
    let account_balance = client.balance_of(account_id);
    assert!(account_balance == 0);
}
