use rocket::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

pub fn rocket() -> Rocket<Build> {
    build().mount("/", routes![index])
}

#[cfg(not(test))]
fn main() {
    if let Err(e) = rocket::execute(rocket().launch()) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}