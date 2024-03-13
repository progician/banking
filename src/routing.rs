use rocket::*;
use rocket::response::status::{BadRequest, NotFound};
use rocket::serde::json::Json;

use uuid::Uuid;
use crate::model::{Account, Model, Money, User};

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}


#[post("/api/user", format = "json", data = "<user>")]
fn create_user(state: &State<Model>, user: Json<User>) -> Result<Json<User>, BadRequest<String>> {
    if user.id.is_some() {
        Err(BadRequest("id must not be provided".to_string()))
    }
    else {
        let new_user = state.new_user(user.first_name.clone(), user.last_name.clone());
        Ok(new_user.into())
    }
}


#[get("/api/user/<user_id>", format = "json")]
fn get_user(state: &State<Model>, user_id: &str) -> Result<Json<User>, NotFound<String>> {
    match state.users.read().unwrap().get(&Uuid::parse_str(user_id).unwrap()) {
        Some(user) => { Ok(user.clone().into()) },
        None => { Err(NotFound(format!("user {} does not exist", user_id))) },
    }
}


#[post("/api/account", format = "json", data = "<account>")]
fn create_account(account: Json<Account>) -> Result<Json<Account>, BadRequest<String>> {
    if account.id.is_some() {
        Err(BadRequest("id must not be provided".to_string()))
    }
    else {
        Ok(Json(Account {
            id: Some(Uuid::new_v4()),
            account_holder: account.account_holder.clone(),
            balance: Money {
                amount: 0,
                currency: "USD".to_string(),
            },
        }))
    }
}


#[get("/api/account/<_account>")]
fn get_account(_account: String) -> Result<Json<Account>, BadRequest<String>> {
    // let account_id = uuid::Uuid::parse_str(&account).map_err(|e| BadRequest(e.to_string()))?;
    Ok(Json(Account {
        id: Some(Uuid::new_v4()),
        account_holder: Uuid::new_v4(),
        balance: Money {
            amount: 0,
            currency: "USD".to_string(),
        },
    }))
}


pub fn rocket() -> Rocket<Build> {
    build()
        .manage(Model::new())
        .mount("/", routes![
            create_user,
            get_user,
            create_account,
            get_account,

            index,
        ])
}
