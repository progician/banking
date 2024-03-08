use rocket::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
