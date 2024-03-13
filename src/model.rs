use std::collections::HashMap;
use std::sync::RwLock;
use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Money {
    pub amount: u64,
    pub currency: String,
}

impl Money {
    pub fn new(amount: u64, currency: &str) -> Money {
        Money {
            amount: amount,
            currency: currency.to_string(),
        }
    }

    pub fn zero(currency: &str) -> Money {
        Money {
            amount: 0,
            currency: currency.to_string(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
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

    pub fn new_with_id(account_holder: Uuid) -> Account {
        Account {
            id: Some(Uuid::new_v4()),
            account_holder: account_holder,
            balance: Money {
                amount: 0,
                currency: "USD".to_string(),
            },
        }
    }
}



#[derive(Serialize, Deserialize)]
pub struct Deposit {
    pub account_id: Uuid,
    pub deposit_value: Money,
}

impl Deposit {
    pub fn new(account_id: &Uuid, deposit_value: Money) -> Deposit {
        Deposit {
            account_id: account_id.clone(),
            deposit_value: deposit_value,
        }
    }
}



pub struct Model {
    pub users: RwLock<HashMap<Uuid, User>>,
    pub accounts: RwLock<HashMap<Uuid, Account>>,
}


impl Model {
    pub fn new() -> Self {
        Model {
            users: RwLock::new(HashMap::new()),
            accounts: RwLock::new(HashMap::new()),
        }
    }

    pub fn new_user(&self, first_name: String, last_name: String) -> User {
        let id = uuid::Uuid::new_v4();
        let new_user_entry = User {
            id: Some(id.clone()),
            first_name: first_name,
            last_name: last_name,
        };

        let mut unlocked_users = self.users.write().unwrap();
        unlocked_users.insert(id, new_user_entry.clone());
        return new_user_entry;
    }

    pub fn new_account(&self, account_holder: Uuid) -> Result<Account, String> {
        self.users.read().unwrap().get(&account_holder).ok_or("account holder not found")?;

        let new_account = Account::new_with_id(account_holder);
        let mut unlocked_accounts = self.accounts.write().unwrap();
        unlocked_accounts.insert(new_account.id.clone().unwrap(), new_account.clone());
        return Ok(new_account);
    }

    pub fn apply_deposit(&self, deposit: Deposit) -> Result<Account, String> {
        let mut unlocked_accounts = self.accounts.write().unwrap();
        let account = unlocked_accounts.get_mut(&deposit.account_id).ok_or("account not found")?;
        account.balance.amount += deposit.deposit_value.amount;
        Ok(account.clone())
    }
}
