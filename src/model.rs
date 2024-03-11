use std::collections::HashMap;
use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
}


impl User {
    pub fn create(first_name: String, last_name: String) -> User {
        User {
            id: None,
            first_name,
            last_name,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct Money {
    pub amount: u64,
    pub currency: String,
}


#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: Option<Uuid>,
    pub account_holder: Uuid,
    pub balance: Money,
}

impl Account {
    pub fn create(account_holder: Uuid) -> Account {
        Account {
            id: None,
            account_holder: account_holder,
            balance: Money {
                amount: 0,
                currency: "USD".to_string(),
            },
        }
    }
}


pub struct Model {
    pub users: HashMap<Uuid, User>,
    pub accounts: HashMap<Uuid, Account>,
}
