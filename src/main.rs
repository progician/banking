use rocket::*;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;

mod model;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}


#[post("/api/user", format = "json", data = "<user>")]
fn create_user(user: Json<model::User>) -> Result<Json<model::User>, BadRequest<String>> {
    if user.id.is_some() {
        Err(BadRequest("id must not be provided".to_string()))
    }
    else {
        Ok(Json(model::User {
            id: Some(uuid::Uuid::new_v4()),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
        }))
    }
}


#[post("/api/account", format = "json", data = "<account>")]
fn create_account(account: Json<model::Account>) -> Result<Json<model::Account>, BadRequest<String>> {
    if account.id.is_some() {
        Err(BadRequest("id must not be provided".to_string()))
    }
    else {
        Ok(Json(model::Account {
            id: Some(uuid::Uuid::new_v4()),
            account_holder: account.account_holder.clone(),
            balance: model::Money {
                amount: 0,
                currency: "USD".to_string(),
            },
        }))
    }
}


#[get("/api/account/<account>")]
fn get_account(account: String) -> Result<Json<model::Account>, BadRequest<String>> {
    // let account_id = uuid::Uuid::parse_str(&account).map_err(|e| BadRequest(e.to_string()))?;
    Ok(Json(model::Account {
        id: Some(uuid::Uuid::new_v4()),
        account_holder: uuid::Uuid::new_v4(),
        balance: model::Money {
            amount: 0,
            currency: "USD".to_string(),
        },
    }))
}


pub fn rocket() -> Rocket<Build> {
    build().mount("/", routes![
        create_user,
        create_account,
        get_account
    ])
}

#[cfg(not(test))]
fn main() {
    if let Err(e) = execute(rocket().launch()) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}