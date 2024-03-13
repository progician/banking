#[cfg(not(test))] use rocket::execute;
#[cfg(not(test))] use banking::routing::rocket;

#[cfg(not(test))]
fn main() {
    if let Err(e) = execute(rocket().launch()) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}