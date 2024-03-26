use assert2::assert;
use rstest::{fixture, rstest};
use rocket::{http::ContentType, local::blocking::Client};
use rocket::http::Status;

use banking::routing::rocket;
use banking::model::{Account, Deposit, Money, User, Widthdrawal};


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


    fn user_for(&self, user_id: uuid::Uuid) -> User {
        let response = self.client.get(format!("/api/user/{}", user_id)).dispatch();
        assert!(response.status() == Status::Ok);
        response.into_json::<User>().unwrap()
    }

    fn deposit(&self, account: &Account, deposit_value: Money) -> Account {
        let response = self.client.post("/api/deposit")
            .header(ContentType::JSON)
            .json(&Deposit::new(&account.id.unwrap(), deposit_value))
            .dispatch();
        assert!(response.status() == Status::Ok);
        response.into_json::<Account>().unwrap()
    }

    fn withdraw(&self, account: &Account, withdrawal_value: Money) -> Result<Account, String> {
        let response = self.client.post("/api/withdraw")
            .header(ContentType::JSON)
            .json(&Widthdrawal::new(&account.id.unwrap(), withdrawal_value))
            .dispatch();
        if response.status() == Status::Ok {
            Ok(response.into_json::<Account>().unwrap())
        }
        else {
            Err(response.into_string().unwrap())
        }
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


#[rstest]
fn account_holder_can_be_identified(client: TestClient) {
    let account = client.create_account();
    let account_holder_user = client.user_for(account.account_holder);

    assert!(account_holder_user.first_name == "John");
    assert!(account_holder_user.last_name == "Doe");
}


#[rstest]
fn deposit_affects_balance(client: TestClient) {
    let account = client.create_account();

    let deposit_value = Money::new(10, "USD");
    let account_after_deposit = client.deposit(&account, deposit_value.clone());

    assert!(account_after_deposit.balance == deposit_value);
}


#[rstest]
fn withdrawal_the_whole_deposit_empties_the_acount_balance(client: TestClient) {
    let account = client.create_account();
    let in_out_money: Money = Money::new(10, "USD");

    client.deposit(&account, in_out_money.clone());
    let account_after_withdrawal = client.withdraw(&account, in_out_money.clone()).unwrap();

    assert!(account_after_withdrawal.balance == Money::zero("USD"));
}

#[rstest]
fn account_cannot_be_overdrawn(client: TestClient) {
    let account = client.create_account();

    let deposit_value: Money = Money::new(10, "USD");
    let withdrawal_value: Money = Money::new(100, "USD");

    client.deposit(&account, deposit_value.clone());
    match client.withdraw(&account, withdrawal_value) {
        Ok(_) => assert!(false),
        Err(message) => assert!(message == "Insufficient funds".to_string())
    }
    assert!(client.balance_of(&account) == deposit_value);
}
