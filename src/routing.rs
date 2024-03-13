use rocket::*;
use rocket::response::status::{BadRequest, NotFound};
use rocket::serde::json::Json;

use uuid::Uuid;
use crate::model::{Account, Deposit, Model, Money, User};

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
fn create_account(state: &State<Model>, account: Json<Account>) -> Result<Json<Account>, BadRequest<String>> {
    if account.id.is_some() {
        Err(BadRequest("id must not be provided".to_string()))
    }    
    else {
        state
            .new_account(account.account_holder.clone())
            .map(Json)
            .map_err(|e| BadRequest(e))
    }
}


#[get("/api/account/<account>")]
fn get_account(state: &State<Model>, account: String) -> Result<Json<Account>, NotFound<String>> {
    let readable_accounts = state.accounts.read().unwrap();
    match readable_accounts.get(&Uuid::parse_str(&account).unwrap()) {
        Some(account) => { Ok(account.clone().into()) },
        None => { Err(NotFound(format!("account {} does not exist", account))) },
    }
}


#[post("/api/deposit", format="json", data="<data>")]
fn deposit(state: &State<Model>, data: Json<Deposit>) -> Result<Json<Account>, BadRequest<String>> {
    state
        .apply_deposit(data.into_inner())
        .map(Json).map_err(|e| BadRequest(e))
}


pub fn rocket() -> Rocket<Build> {
    build()
        .manage(Model::new())
        .mount("/", routes![
            create_user,
            get_user,
            create_account,
            get_account,

            deposit,

            index,
        ])
}
