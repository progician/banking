use assert2::assert;
use rstest::{fixture, rstest};
use rocket::{http::ContentType, local::blocking::Client};
use rocket::http::Status;

#[path = "../src/main.rs"]
mod main;

#[path = "../src/model.rs"]
mod model;

use main::rocket;
use model::{User, Account, Money};


struct TestClient {
    client: Client,
    default_user: User,
}


impl TestClient {
    fn new() -> Self {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let res = client.post("/api/user")
            .header(ContentType::JSON)
            .body(r#"{"first_name": "John", "last_name": "Doe"}"#)
            .dispatch();
        assert!(res.status() == Status::Ok);
        let new_user = res.into_json::<User>().unwrap();

        TestClient {
            client: client,
            default_user: new_user,
        }
    }

    fn create_account(&self) -> Account {
        let account = Account::create(self.default_user.id.expect("or else"));
        let res = self.client.post("/api/account")
            .header(ContentType::JSON)
            .json(&account)
            .dispatch();
        assert!(res.status() == Status::Ok);
        res.into_json::<Account>().unwrap()
    }

    fn balance_of(&self, account: &Account) -> Money {
        let response = self.client.get(format!("/api/account/{}", account.id.expect("or else"))).dispatch();
        assert!(response.status() == Status::Ok);
        response.into_json::<Account>().unwrap().balance
    }
}


#[fixture]
fn client() -> TestClient {
    TestClient::new()
}


#[rstest]
fn newly_created_accounts_are_empty(client: TestClient) {
    let account = client.create_account();
    let account_balance = client.balance_of(&account);
    assert!(account_balance.amount == 0);
}
